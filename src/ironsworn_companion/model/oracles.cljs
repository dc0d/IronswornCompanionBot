(ns ironsworn-companion.model.oracles
  (:require
   [ironsworn-companion.model.dice :refer [roll]]
   [shadow.resource :as rc]))

(def prompts-json-content (rc/inline "./ironsworn_oracles_prompts.json"))
(def prompts (js->clj (.parse js/JSON prompts-json-content) :keywordize-keys false))
(def action-table (let [oracles (get prompts "Oracles")
                        action-item (first (filter (fn [x] (= (get x "Name") "Action")) oracles))]
                    (get action-item "Oracle Table")))
(def theme-table (let [oracles (get prompts "Oracles")
                       action-item (first (filter (fn [x] (= (get x "Name") "Theme")) oracles))]
                   (get action-item "Oracle Table")))

(defn- action []
  (let [[d100] (roll "1d100")]
    (action-table (- d100 1))))

(defn- theme []
  (let [[d100] (roll "1d100")]
    (theme-table (- d100 1))))

(defn action-and-theme []
  (let [action (action)
        theme (theme)]
    (str
     (get action "Description") " " (get theme "Description")
     " 🎲 "
     (get action "Chance") " " (get theme "Chance"))))
