#include <deltachat.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <signal.h>

#define NONE 0
#define INFO 1
#define INFOBANNER "[info] "
#define WARN 2
#define WARNBANNER "[warn] "
#define ERR  3
#define ERRBANNER  "[err] "
#define MAXLOGGER 73 // 80 - max banner width
#define logit(a,b) logger ( a , __FILE__ , __LINE__ , b )
#define strify(a) #a

// global state; only written by main thread, read by both
static long log_level = NONE;
static int exiting_flag = 0;
static dc_context_t *context = 0;
static pthread_t event_thread = 0;

// separation for thread safety
static char thread_buffer80[80];
static char main_buffer80[80];

void logger (int32_t type, char* file, int lineno, char* msg) {
  if (type < log_level) {
    return;
  }
  if (type > NONE) {
    fprintf(stderr, "%s:%d:\n", file, lineno);
  }
  switch (type) {
    case NONE:
      break;
    case INFO:
      fprintf(stderr, INFOBANNER);
      break;
    case WARN:
      fprintf(stderr, WARNBANNER);
      break;
    default:
      fprintf(stderr, ERRBANNER);
  }

  fprintf(stderr, "%s\n", msg);
  fflush(stderr);
}

// cleanly exit
void stop_context_and_exit (int exit_code) {
  // stop the event loop
  logit(INFO, "setting event loop exit flag");
  exiting_flag = 1;

  // stop the io, which sends a message to the event loop
  if (context) {
    logit(INFO, "stopping io");
    dc_stop_io(context);
    dc_stop_ongoing_process(context);
  }

  // join the event loop
  if (event_thread) {
    int thread_join_result = pthread_join(event_thread, NULL);
    if (thread_join_result) {
      logit(WARN, "thread join failed on exit");
    }
  }

  // remove any remaining events from the queue
  if (context) {
    dc_event_emitter_t *emitter = dc_get_event_emitter(context);
    logit(INFO, "cleaning up remaining events");
    dc_context_unref(context);
    while (dc_get_next_event(emitter)) {}
    dc_event_emitter_unref(emitter);
  }

  logit(INFO, "exiting");
  exit(exit_code);
}

// signal handler for SIGINT and SIGTERM
void exit_handler(int signo) {
  snprintf(main_buffer80, MAXLOGGER, "caught signal %d", signo);
  logit(WARN, main_buffer80);
  stop_context_and_exit(0);
}

// handle chat messages
// called by event_thread
void handle_message (int32_t chat_id, int32_t message_id) {
  snprintf(thread_buffer80, MAXLOGGER, 
      "chat/message id: %d/%d", chat_id, message_id);
  logit(INFO, thread_buffer80);

  // dc_get_msg and dc_send_text_msg both are unsigned
  if (chat_id < 0) {
    snprintf(thread_buffer80, MAXLOGGER,
      "chat_id < 0, %d, cannot be sent to dc_get_chat", chat_id);
    logit(WARN, thread_buffer80);
    return;
  }
  if (message_id < 0) {
    snprintf(thread_buffer80, MAXLOGGER,
      "message_id < 0, %d, cannot be sent to dc_get_chat", message_id);
    logit(WARN, thread_buffer80);
    return;
  }

  // process the message
  dc_chat_t *chat = dc_get_chat(context, (uint32_t)chat_id);
  int32_t type = dc_chat_get_type(chat);
  if (type == DC_CHAT_TYPE_SINGLE ||
      type == DC_CHAT_TYPE_GROUP ||
      type == DC_CHAT_TYPE_MAILINGLIST) {

    dc_msg_t *msg = dc_get_msg(context, (uint32_t)message_id);

    if (!msg) {
      logit(WARN, "unable to get message handle");
      return;
    }

    // echo back
    dc_msg_force_plaintext(msg);
    char *text = dc_msg_get_text(msg);
    dc_contact_t *contact = dc_get_contact(context, dc_msg_get_from_id(msg));
    char *addr = dc_contact_get_addr(contact);

    dc_send_text_msg(context, (uint32_t)chat_id, text);

    dc_str_unref(addr);
    dc_contact_unref(contact);
    dc_str_unref(text);
    dc_msg_unref(msg);
  }

  dc_chat_unref(chat);
}

// event loop - separate thread from main
void *event_handler () {

  dc_event_emitter_t *emitter = dc_get_event_emitter(context);
  dc_event_t *event;

  // process events
  while (event = dc_get_next_event(emitter)) {

    // if we got a sigint or sigterm
    if (exiting_flag)
    {
      logit(INFO, "event loop caught exit flag");
      dc_event_unref(event);
      break;
    }

    // process other events
    int event_type = dc_event_get_id(event);
    switch (event_type) {
      case DC_EVENT_ERROR:
      case DC_EVENT_INFO:
      case DC_EVENT_WARNING:
      {
        char *message = dc_event_get_data2_str(event);

        switch (event_type) {
        case DC_EVENT_INFO:
          logit(INFO, message);
          break;
        case DC_EVENT_WARNING:
          logit(WARN, message);
          break;
        default:
          logit(ERR, message);
          break;
        }
        dc_str_unref(message);
      }
      break;

      case DC_EVENT_CONFIGURE_PROGRESS:
      {
        int progress = dc_event_get_data1_int(event);
        char *comment = dc_event_get_data2_str(event);

        snprintf(thread_buffer80, MAXLOGGER, "%d %s", progress, comment);
        logit(INFO, thread_buffer80);

        if (progress <= 0) {
          logit(ERR,
            "configuration failed, check credentials and previous errors");
          stop_context_and_exit(1);
        } else if (progress >= 1000) {
          logit(INFO,
            "successful configuration, starting");
          dc_start_io(context);
        }
        dc_str_unref(comment);
      }
      break;

      case DC_EVENT_INCOMING_MSG:
      {
        int chat_id = dc_event_get_data1_int(event);
        int message_id = dc_event_get_data2_int(event);

        snprintf(thread_buffer80, MAXLOGGER, "%d %d", chat_id, message_id);
        logit(INFO, thread_buffer80);

        handle_message(chat_id, message_id);
      }
      break;

      default:
        snprintf(thread_buffer80, MAXLOGGER,
          "unknown event type %d", event_type);
        logit(WARN, thread_buffer80);
    }

    dc_event_unref(event);
  }
  dc_event_emitter_unref(emitter);

  logit(INFO, "exited event loop");
  return 0;
}

int main() {
  // make sure we have a modern version of deltachat core
  dc_jsonrpc_instance_t* unused;

  // set our log level
  char *loglim = getenv("DELTACHAT_C_ECHOBOT_LOGLEVEL");
  if (loglim) {
    log_level = strtol(loglim, 0, 10);
    snprintf(main_buffer80, MAXLOGGER, "setting log limit to %ld", log_level);
    logit(INFO, main_buffer80);
  }

  // clean exit signal handlers
  if (signal(SIGINT, exit_handler) == SIG_ERR) {
    logit(WARN, "unable to set SIGINT handler");
  }
  if (signal(SIGTERM, exit_handler) == SIG_ERR) {
    logit(WARN, "unable to set SIGTERM handler");
  }

  // event handler loop
  context = dc_context_new(NULL, "./bot.db", NULL);
  if (pthread_create(&event_thread, NULL, event_handler, NULL)) {
    logit(ERR, "thread creation failed");
    stop_context_and_exit(1);
  }

  // start main processing
  if (dc_is_configured(context)) {
    char* addr = dc_get_config (context, "addr");
    snprintf(main_buffer80, MAXLOGGER, "starting %s", addr);
    dc_set_config(context, "bot", "1");
    dc_set_config(context, "fetch_existing_msgs", "0");
  }
  // otherwise set our account up
  else {
    // setup a new account to make the database
    char *addr = getenv("DELTACHAT_C_ECHOBOT_EMAIL");
    char *mailpw = getenv("DELTACHAT_C_ECHOBOT_PASSWORD");

    if (!addr) {
      logit(ERR, "DELTACHAT_C_ECHOBOT_EMAIL environment variable not specified");
      stop_context_and_exit(1);
    }
    if (!mailpw) {
      logit(ERR, "DELTACHAT_C_ECHOBOT_PASSWORD environment variable not specified");
      stop_context_and_exit(1);
    }

    snprintf(main_buffer80, MAXLOGGER, "first time configuring %s", addr);
    logit(INFO, main_buffer80);
    dc_set_config(context, "addr", addr);
    dc_set_config(context, "mail_pw", mailpw);
    dc_set_config(context, "bot", "1");
    dc_set_config(context, "fetch_existing_msgs", "0");
    dc_configure(context);
    logit(INFO, "done configuring");
  }

  // wait for event thread to complete
  dc_start_io(context);
  int thread_join_result = pthread_join(event_thread, NULL);
  if (thread_join_result) {
    logit(WARN, "thread join failed on exit");
  }
  event_thread = (pthread_t)NULL;

  // clean up
  stop_context_and_exit(0);
}
