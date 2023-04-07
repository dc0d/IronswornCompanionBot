(ns ironsworn-companion.actions.action-and-theme
  (:require
   [ironsworn-companion.model.oracles :refer [action-and-theme]]))

(defn handle-action-and-theme [ctx]
  (try
    (let [result (action-and-theme)]
      (.reply ^js ctx result))
    (catch :default err
      (prn (str "fn: handle-action-and-theme error: " err)))))
