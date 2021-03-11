#include <deltachat.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

void handle_message(dc_context_t *context, int chat_id, int message_id) {
  dc_chat_t *chat = dc_get_chat(context, chat_id);

  if (dc_chat_get_type(chat) == DC_CHAT_TYPE_SINGLE) {
    dc_msg_t *msg = dc_get_msg(context, message_id);

    if (!msg) {
      return;
    }

    char *text = dc_msg_get_text(msg);
    dc_send_text_msg(context, chat_id, text);
    dc_str_unref(text);
    dc_msg_unref(msg);
  }

  dc_chat_unref(chat);
}

void *event_handler(void *context) {
  dc_event_emitter_t *emitter = dc_get_event_emitter(context);
  dc_event_t *event;
  while ((event = dc_get_next_event(emitter)) != NULL) {
    // use the event as needed, e.g. dc_event_get_id() returns the type.
    // once you're done, unref the event to avoid memory leakage:
    int event_type = dc_event_get_id(event);

    if (event_type == DC_EVENT_ERROR || event_type == DC_EVENT_INFO ||
        event_type == DC_EVENT_WARNING) {
      char *message = dc_event_get_data2_str(event);

      switch (event_type) {
      case DC_EVENT_ERROR:
        printf("[Error] %s\n", message);
        break;
      case DC_EVENT_INFO:
        printf("[Info] %s\n", message);
        break;
      case DC_EVENT_WARNING:
        printf("[Warn] %s\n", message);
        break;
      }
      dc_str_unref(message);
    } else if (event_type == DC_EVENT_CONFIGURE_PROGRESS) {

      int progress = dc_event_get_data1_int(event);
      char *comment = dc_event_get_data2_str(event);
      printf("[configure-progress] %d %s\n", progress, comment);

      if (progress == 0) {
        // Failed to configure
        printf(
            "[BOT] configuration failed, maybe your credentials are incorect? "
            "look for error messages above and restart the bot to try again");
      } else if (progress == 1000) {
        printf("[BOT] confuguration sucessfull, starting io");
        dc_start_io(context);
      }
      dc_str_unref(comment);

    } else if (event_type == DC_EVENT_MSGS_CHANGED) {
      int chat_id = dc_event_get_data1_int(event);
      int message_id = dc_event_get_data2_int(event);
      printf("[msg-changed] %d %d\n", chat_id, message_id);

      dc_msg_t *msg = dc_get_msg(context, message_id);

      if (dc_msg_get_chat_id(msg) == DC_CHAT_ID_DEADDROP) {
        // check if contact is new / and accept its contact request
        printf("[BOT] accepting new contact\n");
        int new_chat_id = dc_decide_on_contact_request(context, message_id,
                                                       DC_DECISION_START_CHAT);
        handle_message(context, new_chat_id, message_id);
      }
      dc_msg_unref(msg);
    } else if (event_type == DC_EVENT_INCOMING_MSG) {
      int chat_id = dc_event_get_data1_int(event);
      int message_id = dc_event_get_data2_int(event);
      printf("[incoming-msg] %d %d\n", chat_id, message_id);
      handle_message(context, chat_id, message_id);
    } else {
      printf("[?] unhandled event of type: %d\n", event_type);
    }

    dc_event_unref(event);
  }
  dc_event_emitter_unref(emitter);
}

void stop_context(dc_context_t *context) {
  dc_stop_io(context);
  dc_stop_ongoing_process(context);
}

int main() {
  char *addr = getenv("addr");
  char *mailpw = getenv("mailpw");

  printf("starting bot\n");
  dc_context_t *context = dc_context_new(NULL, "deltachat-db/dc.db", NULL);

  static pthread_t event_thread;
  if (pthread_create(&event_thread, NULL, event_handler, context) != 0) {
    printf("Event Thread creation failed\n");
    stop_context(context);
    return 1;
  }

  if (!dc_is_configured(context)) {
    if (!addr) {
      printf("you need to specify the addr enviroment variable to the bots "
             "email address\n");
    }

    if (!mailpw) {
      printf("you need to specify the mailpw enviroment variable to the bots "
             "email password\n");
    }
    if (!addr || !mailpw) {
      stop_context(context);
      printf("shutting down...\n");
      int thread_join_result = pthread_join(event_thread, NULL);
      if (thread_join_result != 0) {
        printf("join thread failed with error code %d\n", thread_join_result);
      }
      return 1;
    }

    printf("configuring bot\n");
    dc_set_config(context, "addr", addr);
    dc_set_config(context, "mail_pw", mailpw);
    dc_set_config(context, "bot", "1");
    dc_set_config(context, "fetch_existing_msgs", "0");
    dc_configure(context);
  } else {
    printf("already configured, wait for messages\n");
    dc_start_io(context);
  }

  // wait for event thread to complete
  int thread_join_result = pthread_join(event_thread, NULL);
  if (thread_join_result != 0) {
    printf("join thread failed with error code %d\n", thread_join_result);
  }

  return 0;
}
