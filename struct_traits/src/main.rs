fn main() {
    let article = NewsArticle::instancenewsarticle("Seven ages of men".to_string(), "Shakespere".to_string());
    let t = Tweet::instance_tweet("abc".to_string(), "xyz".to_string());
    let t2 = Tweet::create_instance("abc".to_string(), "xyz".to_string());
    let article2 = NewsArticle::create_instance("abc".to_string(), "xyz".to_string());
    println!("{:?}",article2);
    println!("{:?}",t2);

    impl_function(article,t);
    
}
#[derive(Debug)]
struct NewsArticle{
    name:String,
    author:String
}
trait Instances<T>{
    fn create_instance(item1:String,item2:String)->T;
}//this trait function will generate instance of that struct that implements it
impl Instances<NewsArticle> for NewsArticle{
    fn create_instance(name: String, author: String)->NewsArticle{
        NewsArticle{
            name,
            author
        }
    }
}
#[derive(Debug)]
struct Tweet {
    username:String,
    content:String
}
impl Instances<Tweet> for Tweet{
    fn create_instance(username:String, content: String)->Tweet{
        Tweet{
            username,
            content
        }
    }
}
trait Summary{
    fn summarize(&self)->String;
   
}
impl Summary for Tweet{
    fn summarize(&self)->String{
        format!("username: {} content: {}",self.username,self.content)
    }

}
impl Summary for NewsArticle{
    fn summarize(&self)->String{
        format!("name: {} author: {}",self.name,self.author)
    }
}
fn impl_function<T: Summary,U:Summary>(item1:T,item2:U){
    println!("value1: \n {}\n value2: \n {}",item1.summarize(),item2.summarize());
}
impl Tweet{
    fn instance_tweet (username: String , content:String)->Tweet{
        Tweet{
            username,
            content
        }
    }
}
impl NewsArticle{
    fn instancenewsarticle (name: String , author:String)->NewsArticle{
        NewsArticle{
            name,
            author
        }
    }
}