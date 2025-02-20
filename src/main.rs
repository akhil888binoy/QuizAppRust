use std::io::stdin;
#[derive(Debug)] 



struct Question{
    question : String ,
    options :  Vec<String>,
    answer: String,
}

fn getInput(path : &str)-> Option<String> {
    println!("{}", path);
    let mut input = String :: new();
    stdin().read_line(&mut input).expect("cannot read");
    return  Some(input.trim().to_string());
}

fn AddOptions(options : &mut Vec<String>) {
    for i in 0..3{
        let option = getInput("Enter the option").unwrap();
        options.push(option);
    }
   
}
fn AddQuestion (questions : &mut Vec<Question>){
    let question  = getInput("Give a question").unwrap();
    let mut options = Vec :: with_capacity(3);
    AddOptions(&mut options);
    let answer = getInput("Give me answer of this question").unwrap();
    let questionpush = Question{
        question,
        options,
        answer
    } ;
    questions.push(questionpush);
}


fn ShowQuestions(questions : &mut Vec<Question>){
    println!("Questions and Answers");
    for question in questions.iter(){
        println!("{:?}", question);

    }
}

fn ShowQuestionsWNAns(questions : &mut Vec<Question> , score : &mut u32){
    for question in questions.iter(){
        println!("{:?} \n {:?}", question.question , question.options);
        let chooseoption = getInput("Choose an option").unwrap();
        if chooseoption == question.answer {
            println!("You choose correct option");
            *score +=1;
            println!("Score : {} " , score);
            continue;
        }else {
            println!("You choose wrong option");
            continue;
        }
    }
}

fn main() {
    let mut questions : Vec<Question> = Vec:: new();
    let mut score : u32 = 0;
    loop{
        println!(" Press 1 if you are Admin \n Press 2 if you are Contestant \n Press 3 if you want to Exit ");
        let choice: u32 = getInput(" Give your Role").unwrap().trim().parse().unwrap();
        match choice{
            1 => {
                println!("So you are Admin");
                ShowQuestions(&mut questions);
                AddQuestion(&mut questions);
                println!("Add Question and Answer");
                continue;
            },
            2=>{
                println!("So you are the Contestant");
                ShowQuestionsWNAns(&mut questions , &mut score);
                continue;
            },
            3=>{
                println!("Bye Bye ............");
                break;
            }
            _ =>{
                println!("Enter a valid choice ");
                continue;
            }
        }
    }
}
