(ns clj.core
  (:gen-class)
  (:require [clojure.string :as str]))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println "Hello, World!"))


(def *day-one* (str/split (slurp "../1.input") #"\n"))
(def *test-data* ["1721" "979" "366" "299" "675" "1456"])

(defn one-a
  []
  (last (for [i (map #(Integer/parseInt %) *day-one*)
              j (map #(Integer/parseInt %) *day-one*)
              :let [n (* i j)]
              :when (= (+ i j) 2020)] n)))

(defn one-b
  []
  (last (for [i (map #(Integer/parseInt %) *day-one*)
              j (map #(Integer/parseInt %) *day-one*)
              k (map #(Integer/parseInt %) *day-one*)
              :let [n (* i j k)]
              :when (= (+ i j k) 2020)] n)))



