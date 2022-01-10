fn main() {
   let sentence = "This is an f  abnormal sentence".to_string();

   println!("{}",pirat_lang(sentence));
}

fn pirat_lang(sentence: String) -> String{
   let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
   let mut new_sentence= String::new();
   for word in sentence.split_whitespace(){
         if vowels.contains(&word.chars().next().unwrap()) == true{
            new_sentence = format!("{}{}-hay ", new_sentence ,word);
            continue;
         }
         if word.len() == 1{
            new_sentence = format!("{}{}ay ", new_sentence, word);
            continue;
         }
         new_sentence = format!("{}{}-{}ay ", new_sentence, &word[1..], &word[0..1]);
   }
   new_sentence
}