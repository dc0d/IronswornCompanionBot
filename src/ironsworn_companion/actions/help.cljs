(ns ironsworn-companion.actions.help)

(defn handle-help
  ([ctx msg]
   (.reply ^js ctx (str msg "\n\n" "HELP")))
  ([ctx]
   (.reply ^js ctx "HELP")))
