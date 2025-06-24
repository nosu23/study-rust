// 構造体の定義
struct Person {
    name: String,
    age: u32,
    city: String,
}

// 構造体の実装ブロック
impl Person {
    // 新しいPersonインスタンスを作成する関数
    fn new(name: String, age: u32, city: String) -> Person {
        Person { name, age, city }
    }
    
    // 自己紹介をするメソッド
    fn introduce(&self) {
        println!("こんにちは！私は{}です。{}歳で、{}に住んでいます。", 
                 self.name, self.age, self.city);
    }
    
    // 年齢を更新するメソッド
    fn have_birthday(&mut self) {
        self.age += 1;
        println!("{}さん、お誕生日おめでとうございます！{}歳になりました。", 
                 self.name, self.age);
    }
}

fn main() {
    println!("=== 構造体のサンプル ===");
    
    // 構造体のインスタンスを作成
    let mut person1 = Person::new(
        String::from("田中太郎"), 
        25, 
        String::from("東京")
    );
    
    // メソッドを呼び出し
    person1.introduce();
    
    // 誕生日を祝う
    person1.have_birthday();
    
    // 再度自己紹介
    person1.introduce();
    
    println!("\n=== 別のインスタンス ===");
    
    // 別のインスタンスを作成
    let person2 = Person {
        name: String::from("佐藤花子"),
        age: 30,
        city: String::from("大阪"),
    };
    
    person2.introduce();
    
    println!("\n=== 構造体のフィールドアクセス ===");
    println!("{}さんの年齢: {}歳", person2.name, person2.age);
}
