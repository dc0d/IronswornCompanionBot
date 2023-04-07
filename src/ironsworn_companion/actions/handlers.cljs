(ns ironsworn-companion.actions.handlers
  (:require [clojure.string :refer [starts-with?]]

            [ironsworn-companion.actions.ask-the-oracle :as action-ask]

            [ironsworn-companion.model.definitions :refer [CQPX-ORCL-ATO]]))

(defn handle-callback-query [ctx]
  (try (let [data ctx.callbackQuery.data]
         (cond
           (starts-with? data CQPX-ORCL-ATO) (action-ask/handle-callback-query ctx data)
           :else (throw (js/Error. (str "unknown callback query" data)))))
       (catch :default err
         (prn (str "fn: handle-callback-query error: " err)))))
