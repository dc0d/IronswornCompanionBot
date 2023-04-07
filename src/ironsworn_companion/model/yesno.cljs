(ns ironsworn-companion.model.yesno)

(def yes-text "Yes")
(def no-text "No")

(def to-yesno {:yes :yes
               true :yes
               :no :no
               false :no})

(defn normalize [v]
  (let [yn (to-yesno v)]
    (if (nil? yn)
      :no
      yn)))

(defn to-string [yn]
  (if (= :yes (normalize yn))
    yes-text
    no-text))
