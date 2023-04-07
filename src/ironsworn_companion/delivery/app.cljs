(ns ironsworn-companion.delivery.app
  (:require ["telegraf" :refer [Telegraf]]
            ["telegraf/filters" :refer [message]]
            [ironsworn-companion.actions.action-and-theme :refer [handle-action-and-theme]]
            [ironsworn-companion.actions.ask-the-oracle :refer [handle-ask-the-oracle]]
            [ironsworn-companion.actions.handlers :refer [handle-callback-query]]
            [ironsworn-companion.actions.help :refer [handle-help]]
            [ironsworn-companion.actions.roll :refer [handle-roll]]
            [ironsworn-companion.actions.text-message :refer [handle-text-message]]))

(defn- call-after [f ms]
  (js/setTimeout f ms))
(defn- exit-after [ms]
  (call-after  (fn [] (.exit js/process 0)) ms))

(defn enable-graceful-stop [bot]
  (.once js/process "SIGTERM" (fn []
                                (prn "SIGTERM received")
                                (.stop bot "SIGTERM")
                                (exit-after 500)))
  (.once js/process "SIGINT" (fn []
                               (prn "SIGINT received")
                               (.stop bot "SIGINT")
                               (exit-after 500))))

(defn- read-bot-token []
  (.-BOT_TOKEN (.-env js/process)))

(defn main [& cli-args]
  (let [token (read-bot-token)
        bot (Telegraf. token)]

    (.command bot "help" handle-help)
    (.command bot "roll" handle-roll)
    (.command bot "action_and_theme" handle-action-and-theme)
    (.command bot "ask_the_oracle" handle-ask-the-oracle)

    (.on bot (message "text") handle-text-message)
    (.on bot "callback_query" handle-callback-query)

    (.launch bot)
    (enable-graceful-stop bot)
    0))
