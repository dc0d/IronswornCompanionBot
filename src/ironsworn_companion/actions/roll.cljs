(ns ironsworn-companion.actions.roll
  (:require [clojure.string :refer [join]]

            [ironsworn-companion.actions.utl :refer [message-text]]
            [ironsworn-companion.model.dice :refer [extract is-random-event
                                                    is-strong-hit roll]]))

(defn handle-roll [ctx]
  (try (let [[msg text] (message-text ctx)
             extracted-dice (extract text)]
         (prn "received roll command" [msg])

         (if (= "" extracted-dice)
           (let [[d6 d10-1 d10-2] (roll "1d6,1d10,1d10")
                 text10s (str d10-1 " - " d10-2)
                 strong-hit (if (is-strong-hit d6 d10-1 d10-2) " 💪" "")
                 random-event (if (is-random-event d10-1 d10-2) " 🎁" "")
                 result (str d6 " 🎲 " text10s strong-hit random-event)]
             (.reply ^js ctx result))

           (let [dice-values  (roll extracted-dice)
                 result (join ", " dice-values)]
             (.reply ^js ctx result))))
       (catch :default err
         (prn (str "fn: handle-roll error: " err)))))
