(ns ironsworn-companion.actions.utl)

(defn message-text [ctx]
  (let [jsmsg ctx.message
        msg (js->clj jsmsg :keywordize-keys true)
        text (:text msg)]
    [msg text]))
