(ns ironsworn-companion.model.ask-the-oracle
  (:require [ironsworn-companion.model.dice :refer [roll]]))

(def small-chance "Small Chance")
(def unlikely "Unlikely")
(def fifty-fifty "50/50")
(def likely "Likely")
(def almost-certain "Almost Certain")

(def odds-list [:small-chance
                :unlikely
                :fifty-fifty
                :likely
                :almost-certain])
(def lower-limit {:small-chance 91
                  :unlikely 76
                  :fifty-fifty 51
                  :likely 26
                  :almost-certain 11})
(def to-text {:small-chance small-chance
              :unlikely unlikely
              :fifty-fifty fifty-fifty
              :likely likely
              :almost-certain almost-certain})
(def from-text {small-chance :small-chance
                unlikely :unlikely
                fifty-fifty :fifty-fifty
                likely :likely
                almost-certain :almost-certain})

(defn- between [x min max]
  (and (>= x min) (<= x max)))

(defn- above [x min]
  (between x min 100))

(defn resolve-odds [odds]
  (let [[d100] (roll "1d100")]
    [(if (above d100 (lower-limit odds)) :yes :no) d100]))
