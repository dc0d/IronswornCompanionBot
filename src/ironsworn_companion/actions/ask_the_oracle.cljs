(ns ironsworn-companion.actions.ask-the-oracle
  (:require ["telegraf" :refer [Markup]]
            [clojure.string :refer [replace]]

            [ironsworn-companion.model.ask-the-oracle :as ask]
            [ironsworn-companion.model.definitions :refer [CQPX-ORCL-ATO]]
            [ironsworn-companion.model.yesno :as yn]))

(defn- make-ask-the-oracle-keyboard []
  (let [odds-list ask/odds-list
        data-prefix CQPX-ORCL-ATO
        buttons-seq (map (fn [odds]
                           (let [odds-text (odds ask/to-text)
                                 data (str data-prefix   odds-text)
                                 text odds-text]
                             (Markup.button.callback text data false)))
                         odds-list)
        buttons-coll (partition-all 2 buttons-seq)
        buttons (clj->js buttons-coll)]
    (Markup.inlineKeyboard buttons)))

(defn handle-ask-the-oracle [ctx]
  (try (let [keyboard (make-ask-the-oracle-keyboard)]
         (.replyWithMarkdownV2 ^js ctx "Ask the Oracle" keyboard))
       (catch :default err
         (prn (str "fn: handle-ask-the-oracle error: " err)))))

(defn handle-callback-query [ctx data]
  (try (let [message-id ctx.callbackQuery.message.message_id
             data-prefix CQPX-ORCL-ATO
             odds-text (replace data data-prefix "")
             odds (get ask/from-text odds-text)
             [result rolled] (ask/resolve-odds odds)]
         (.answerCbQuery ^js ctx)
         (.deleteMessage ^js ctx message-id)
         (.reply ^js ctx (str odds-text ": " (yn/to-string result) " 🎲 " rolled)))
       (catch :default err
         (prn (str "fn: handle-callback-query error: " err)))))
