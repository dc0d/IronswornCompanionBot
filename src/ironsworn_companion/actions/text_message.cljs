(ns ironsworn-companion.actions.text-message
  (:require [clojure.string :refer [lower-case]]

            [ironsworn-companion.actions.help :refer [handle-help]]
            [ironsworn-companion.actions.utl :refer [message-text]]

            [ironsworn-companion.model.definitions :refer [UNKNOWN-INPUT]]))

(defn handle-text-message [ctx]
  (try
    (let [[msg text] (message-text ctx)
          lower-text (lower-case text)]
      (prn "received text message" msg)
      (condp = lower-text
        "help" (handle-help ctx)
        (let [txt (str UNKNOWN-INPUT ": " text)]
          (prn txt)
          (handle-help ctx txt))))
    (catch :default err
      (prn (str "fn: text-message-handler error: " err)))))