(ns ironsworn-companion.model.dice
  (:require ["dparse" :refer [parseExprList]]
            [clojure.string :refer [join]]))

(defn is-strong-hit [d6 d10-1 d10-2]
  (and (> d6 d10-1) (> d6 d10-2)))

(defn is-random-event [d10-1 d10-2]
  (= d10-1 d10-2))

(defn extract [text]
  (let [dice-phrases (re-seq #"(\d+d\d+)" text)
        dice-texts (map (fn [[d]] d) dice-phrases)]
    (join "," dice-texts)))

(defn roll [text]
  (let [rolled (parseExprList text)]
    (map (fn [x] (.-value (.eval x))) rolled)))
