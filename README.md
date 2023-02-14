# algo-method
このリポジトリは、私がアルゴ式上で解いた問題のコードを備忘録として残す場となります。  

[アルゴ式](https://algo-method.com/)とは？  
・『プログラミングや情報科学をコツコツ学べる「教科書」』  
・『学んだ内容をゲーム感覚で大量に実践できる「練習問題」』  
の２つで構成される、Web上で完結した学習コンテンツです。(公式サイトより引用)  

このサイトを利用することで、各言語における処理方法や  
AtCoder等のオンラインジャッジサイトで必要となるアルゴリズムへの理解が深まると考えています。

# 環境
* Rust(1.42.0)  
標準入力の取得およびパースについては、  
[Rustで競技プログラミングの入力をスッキリ記述するマクロ](https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8)  
こちらの記事のものをmutableな変数でも使用できるようにして使用中。

# 目次
* <details>
  <summary>入力を受け取る</summary>
  
  * [画面に値を表示する](https://github.com/shho-fi/algomethod/tree/main/src/%E5%85%A5%E5%8A%9B%E3%82%92%E5%8F%97%E3%81%91%E5%8F%96%E3%82%8B/1_%E7%94%BB%E9%9D%A2%E3%81%AB%E5%80%A4%E3%82%92%E8%A1%A8%E7%A4%BA%E3%81%99%E3%82%8B%20(%E6%A8%99%E6%BA%96%E5%87%BA%E5%8A%9B))
  * [値を受け取る](https://github.com/shho-fi/algomethod/tree/main/src/%E5%85%A5%E5%8A%9B%E3%82%92%E5%8F%97%E3%81%91%E5%8F%96%E3%82%8B/2_%E5%80%A4%E3%82%92%E5%8F%97%E3%81%91%E5%8F%96%E3%82%8B%20(%E6%A8%99%E6%BA%96%E5%85%A5%E5%8A%9B))
  * [複数の入力を受け取る](https://github.com/shho-fi/algomethod/tree/main/src/%E5%85%A5%E5%8A%9B%E3%82%92%E5%8F%97%E3%81%91%E5%8F%96%E3%82%8B/3_%E8%A4%87%E6%95%B0%E3%81%AE%E5%85%A5%E5%8A%9B%E3%82%92%E5%8F%97%E3%81%91%E5%8F%96%E3%82%8B)
  * [たくさんの入力を受け取る](https://github.com/shho-fi/algomethod/tree/main/src/%E5%85%A5%E5%8A%9B%E3%82%92%E5%8F%97%E3%81%91%E5%8F%96%E3%82%8B/4_%E3%81%9F%E3%81%8F%E3%81%95%E3%82%93%E3%81%AE%E5%85%A5%E5%8A%9B%E3%82%92%E5%8F%97%E3%81%91%E5%8F%96%E3%82%8B)
</details>

* <details>
  <summary>全探索</summary>
  
  * [配列の全探索](https://github.com/shho-fi/algomethod/tree/main/src/%E5%85%A8%E6%8E%A2%E7%B4%A2/1_%E9%85%8D%E5%88%97%E3%81%AE%E5%85%A8%E6%8E%A2%E7%B4%A2)
  * [数値の全探索](https://github.com/shho-fi/algomethod/tree/main/src/%E5%85%A8%E6%8E%A2%E7%B4%A2/2_%E6%95%B0%E5%80%A4%E3%81%AE%E5%85%A8%E6%8E%A2%E7%B4%A2)
  * [文字列の全探索](https://github.com/shho-fi/algomethod/tree/main/src/%E5%85%A8%E6%8E%A2%E7%B4%A2/3_%E6%96%87%E5%AD%97%E5%88%97%E3%81%AE%E5%85%A8%E6%8E%A2%E7%B4%A2)
  * [二重ループの全探索](https://github.com/shho-fi/algomethod/tree/main/src/%E5%85%A8%E6%8E%A2%E7%B4%A2/4_%E4%BA%8C%E9%87%8D%E3%83%AB%E3%83%BC%E3%83%97%E3%81%AE%E5%85%A8%E6%8E%A2%E7%B4%A2)
  * [複数の配列の全探索](https://github.com/shho-fi/algomethod/tree/main/src/%E5%85%A8%E6%8E%A2%E7%B4%A2/5_%E8%A4%87%E6%95%B0%E3%81%AE%E9%85%8D%E5%88%97%E3%81%AE%E5%85%A8%E6%8E%A2%E7%B4%A2)
  * [ペアの全探索](https://github.com/shho-fi/algomethod/tree/main/src/%E5%85%A8%E6%8E%A2%E7%B4%A2/6_%E3%83%9A%E3%82%A2%E3%81%AE%E5%85%A8%E6%8E%A2%E7%B4%A2)
</details>


* <details>
  <summary>計算量</summary>
  
  * [工夫(1) - for文のループを減らす](https://github.com/shho-fi/algomethod/tree/main/src/%E8%A8%88%E7%AE%97%E9%87%8F/%E5%B7%A5%E5%A4%AB(1)%20-%20for%E6%96%87%E3%81%AE%E3%83%AB%E3%83%BC%E3%83%97%E3%82%92%E6%B8%9B%E3%82%89%E3%81%99)
  * [工夫(2) - 累積和](https://github.com/shho-fi/algomethod/tree/main/src/%E8%A8%88%E7%AE%97%E9%87%8F/%E5%B7%A5%E5%A4%AB(2)%20-%20%E7%B4%AF%E7%A9%8D%E5%92%8C)
</details>

* <details>
  <summary>さまざまな問題</summary>
  
  * [同じ部屋にいた時間](https://github.com/shho-fi/algomethod/blob/main/src/%E3%81%95%E3%81%BE%E3%81%96%E3%81%BE%E3%81%AA%E5%95%8F%E9%A1%8C/01%20%E5%90%8C%E3%81%98%E9%83%A8%E5%B1%8B%E3%81%AB%E3%81%84%E3%81%9F%E6%99%82%E9%96%93.rs)
  * [4と6のFizz Buzz](https://github.com/shho-fi/algomethod/blob/main/src/%E3%81%95%E3%81%BE%E3%81%96%E3%81%BE%E3%81%AA%E5%95%8F%E9%A1%8C/02%204%E3%81%A86%E3%81%AEFizz%20Buzz.rs)
  * [市松模様](https://github.com/shho-fi/algomethod/blob/main/src/%E3%81%95%E3%81%BE%E3%81%96%E3%81%BE%E3%81%AA%E5%95%8F%E9%A1%8C/03%20%E5%B8%82%E6%9D%BE%E6%A8%A1%E6%A7%98.rs)
  * [うるう年判定](https://github.com/shho-fi/algomethod/blob/main/src/%E3%81%95%E3%81%BE%E3%81%96%E3%81%BE%E3%81%AA%E5%95%8F%E9%A1%8C/04%20%E3%81%86%E3%82%8B%E3%81%86%E5%B9%B4%E5%88%A4%E5%AE%9A.rs)
  * [勤怠管理システム](https://github.com/shho-fi/algomethod/blob/main/src/%E3%81%95%E3%81%BE%E3%81%96%E3%81%BE%E3%81%AA%E5%95%8F%E9%A1%8C/05%20%E5%8B%A4%E6%80%A0%E7%AE%A1%E7%90%86%E3%82%B7%E3%82%B9%E3%83%86%E3%83%A0.rs)
  * [注文](https://github.com/shho-fi/algomethod/blob/main/src/%E3%81%95%E3%81%BE%E3%81%96%E3%81%BE%E3%81%AA%E5%95%8F%E9%A1%8C/06%20%E6%B3%A8%E6%96%87.rs)
  * [約分](https://github.com/shho-fi/algomethod/blob/main/src/%E3%81%95%E3%81%BE%E3%81%96%E3%81%BE%E3%81%AA%E5%95%8F%E9%A1%8C/07%20%E7%B4%84%E5%88%86.rs)
  * [アイスの棒(Easy)](https://github.com/shho-fi/algomethod/blob/main/src/%E3%81%95%E3%81%BE%E3%81%96%E3%81%BE%E3%81%AA%E5%95%8F%E9%A1%8C/08%20%E3%82%A2%E3%82%A4%E3%82%B9%E3%81%AE%E6%A3%92(Easy).rs)
  * [曜日計算](https://github.com/shho-fi/algomethod/blob/main/src/%E3%81%95%E3%81%BE%E3%81%96%E3%81%BE%E3%81%AA%E5%95%8F%E9%A1%8C/09%20%E6%9B%9C%E6%97%A5%E8%A8%88%E7%AE%97.rs)
  * [二番目に大きい値](https://github.com/shho-fi/algomethod/blob/main/src/%E3%81%95%E3%81%BE%E3%81%96%E3%81%BE%E3%81%AA%E5%95%8F%E9%A1%8C/10%20%E4%BA%8C%E7%95%AA%E7%9B%AE%E3%81%AB%E5%A4%A7%E3%81%8D%E3%81%84%E5%80%A4.rs)
</details>
