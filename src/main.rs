use futures::StreamExt;
use telegram_bot::*;
use rand::prelude::*;
use dotenv::dotenv;

#[derive(PartialEq)]
enum Gamestate{
    CPUwin,
    Nowin,
    Playerwin,
    Draw
}

fn statecheck<'a>(b_ownvec: Vec<&'a str>, cpuchoice: &'a str, playerchoice: &'a str) -> (Vec<&'a str>, Gamestate){

    let diag1 = vec![b_ownvec[0], b_ownvec[4], b_ownvec[8]];
    let diag2 = vec![b_ownvec[2], b_ownvec[4], b_ownvec[6]];
    let vert1 = vec![b_ownvec[0], b_ownvec[3], b_ownvec[6]];
    let vert2 = vec![b_ownvec[1], b_ownvec[4], b_ownvec[7]];
    let vert3 = vec![b_ownvec[2], b_ownvec[5], b_ownvec[8]];

    if b_ownvec[0..3] == [playerchoice, playerchoice, playerchoice] || b_ownvec[3..=5] == [playerchoice, playerchoice, playerchoice] || 
       b_ownvec[6..=8] == [playerchoice, playerchoice, playerchoice] || diag1  == [playerchoice, playerchoice, playerchoice] ||
       diag2  == [playerchoice, playerchoice, playerchoice] || vert1  == [playerchoice, playerchoice, playerchoice] ||
       vert2  == [playerchoice, playerchoice, playerchoice] || vert3  == [playerchoice, playerchoice, playerchoice]{
        return (b_ownvec, Gamestate::Playerwin);

    } else if b_ownvec[0..3] == [cpuchoice, cpuchoice, cpuchoice] || b_ownvec[3..=5] == [cpuchoice, cpuchoice, cpuchoice] || 
       b_ownvec[6..=8] == [cpuchoice, cpuchoice, cpuchoice] || diag1  == [cpuchoice, cpuchoice, cpuchoice] ||
       diag2 == [cpuchoice, cpuchoice, cpuchoice] || vert1  == [cpuchoice, cpuchoice, cpuchoice]||
       vert2  == [cpuchoice, cpuchoice, cpuchoice] || vert3  == [cpuchoice, cpuchoice, cpuchoice]{
        return (b_ownvec, Gamestate::CPUwin);

    } else {
        return (b_ownvec, Gamestate::Nowin);
    }
}

fn xoalg<'a>(turnsvec: &Vec<&'a str>, cpuchoice: &'a str, playerchoice: &'a str) -> (Vec<&'a str>, Gamestate){
    let mut ownvec = turnsvec.to_owned();
    let diag1 = vec![turnsvec[0], turnsvec[4], turnsvec[8]];
    let diag2 = vec![turnsvec[2], turnsvec[4], turnsvec[6]];
    let vert1 = vec![turnsvec[0], turnsvec[3], turnsvec[6]];
    let vert2 = vec![turnsvec[1], turnsvec[4], turnsvec[7]];
    let vert3 = vec![turnsvec[2], turnsvec[5], turnsvec[8]];
    if playerchoice == "\u{274c}"{
        match turnsvec[0..=2]{
            ["\u{274c}", "\u{274c}", "\u{2b1c}"] | 
            ["\u{274c}", "\u{2b1c}", "\u{274c}"] | 
            ["\u{2b1c}", "\u{274c}", "\u{274c}"] => {
                let index = turnsvec[0..=2].iter().position(|&r| r == "\u{2b1c}").unwrap();
                let _ = std::mem::replace(&mut ownvec[index], cpuchoice);
                return statecheck(ownvec, cpuchoice, playerchoice);
            }
             _ => {}
        }
        match turnsvec[3..=5]{
            ["\u{274c}", "\u{274c}", "\u{2b1c}"] | 
            ["\u{274c}", "\u{2b1c}", "\u{274c}"] | 
            ["\u{2b1c}", "\u{274c}", "\u{274c}"]  => {
                let index = turnsvec[3..=5].iter().position(|&r| r == "\u{2b1c}").unwrap()+3;
                println!("{}", index);
                let _ = std::mem::replace(&mut ownvec[index], cpuchoice);
                return statecheck(ownvec, cpuchoice, playerchoice);
             }
             _ => {}
        }
        match turnsvec[6..=8]{
            ["\u{274c}", "\u{274c}", "\u{2b1c}"] | 
            ["\u{274c}", "\u{2b1c}", "\u{274c}"] | 
            ["\u{2b1c}", "\u{274c}", "\u{274c}"]  => {
                let index = turnsvec[6..=8].iter().position(|&r| r == "\u{2b1c}").unwrap()+6;
                println!("{}", index);
                let _ = std::mem::replace(&mut ownvec[index], cpuchoice);
                return statecheck(ownvec, cpuchoice, playerchoice);
             }
             _ => {}
        }
        match diag1[..]{
            ["\u{274c}", "\u{274c}", "\u{2b1c}"] | 
            ["\u{274c}", "\u{2b1c}", "\u{274c}"] | 
            ["\u{2b1c}", "\u{274c}", "\u{274c}"]  => {
                let index = diag1.iter().position(|&r| r == "\u{2b1c}").unwrap()*4;
                let _ = std::mem::replace(&mut ownvec[index], cpuchoice);
                return statecheck(ownvec, cpuchoice, playerchoice);
             }
             _ => {}
        }
        match diag2[..]{
            ["\u{274c}", "\u{274c}", "\u{2b1c}"] | 
            ["\u{274c}", "\u{2b1c}", "\u{274c}"] | 
            ["\u{2b1c}", "\u{274c}", "\u{274c}"]  => {
                let index = diag2.iter().position(|&r| r == "\u{2b1c}").unwrap()*2+2;
                let _ = std::mem::replace(&mut ownvec[index], cpuchoice);
                return statecheck(ownvec, cpuchoice, playerchoice);
             }
             _ => {}
        }
        match vert1[..]{
            ["\u{274c}", "\u{274c}", "\u{2b1c}"] | 
            ["\u{274c}", "\u{2b1c}", "\u{274c}"] | 
            ["\u{2b1c}", "\u{274c}", "\u{274c}"]  => {
                let index = vert1.iter().position(|&r| r == "\u{2b1c}").unwrap()*3;
                let _ = std::mem::replace(&mut ownvec[index], cpuchoice);
                return statecheck(ownvec, cpuchoice, playerchoice);
             }
             _ => {}
        }
        match vert2[..]{
            ["\u{274c}", "\u{274c}", "\u{2b1c}"] | 
            ["\u{274c}", "\u{2b1c}", "\u{274c}"] | 
            ["\u{2b1c}", "\u{274c}", "\u{274c}"]  => {
                let index = vert2.iter().position(|&r| r == "\u{2b1c}").unwrap()*3+1;
                let _ = std::mem::replace(&mut ownvec[index], cpuchoice);
                return statecheck(ownvec, cpuchoice, playerchoice);
             }
             _ => {}
        }
        match vert3[..]{
            ["\u{274c}", "\u{274c}", "\u{2b1c}"] | 
            ["\u{274c}", "\u{2b1c}", "\u{274c}"] | 
            ["\u{2b1c}", "\u{274c}", "\u{274c}"]  => {
                let index = vert3.iter().position(|&r| r == "\u{2b1c}").unwrap()*3+2;
                let _ = std::mem::replace(&mut ownvec[index], cpuchoice);
                return statecheck(ownvec, cpuchoice, playerchoice);
             }
             _ => {}
        }
    } else{
        match turnsvec[0..=2]{
            ["\u{2b55}", "\u{2b55}", "\u{2b1c}"] | 
            ["\u{2b55}", "\u{2b1c}", "\u{2b55}"] | 
            ["\u{2b1c}", "\u{2b55}", "\u{2b55}"] => {
                let index = turnsvec[0..=2].iter().position(|&r| r == "\u{2b1c}").unwrap();
                let _ = std::mem::replace(&mut ownvec[index], cpuchoice);
                return statecheck(ownvec, cpuchoice, playerchoice);
             }
             _ => {}
        }
        match turnsvec[3..=5]{
            ["\u{2b55}", "\u{2b55}", "\u{2b1c}"] | 
            ["\u{2b55}", "\u{2b1c}", "\u{2b55}"] | 
            ["\u{2b1c}", "\u{2b55}", "\u{2b55}"] => {
                let index = turnsvec[3..=5].iter().position(|&r| r == "\u{2b1c}").unwrap()+3;
                let _ = std::mem::replace(&mut ownvec[index], cpuchoice);
                return statecheck(ownvec, cpuchoice, playerchoice);
             }
             _ => {}
        }
        match turnsvec[6..=8]{
            ["\u{2b55}", "\u{2b55}", "\u{2b1c}"] | 
            ["\u{2b55}", "\u{2b1c}", "\u{2b55}"] | 
            ["\u{2b1c}", "\u{2b55}", "\u{2b55}"] => {
                let index = turnsvec[6..=8].iter().position(|&r| r == "\u{2b1c}").unwrap()+6;
                let _ = std::mem::replace(&mut ownvec[index], cpuchoice);
                return statecheck(ownvec, cpuchoice, playerchoice);
             }
             _ => {}
        }
        match diag1[..]{
            ["\u{2b55}", "\u{2b55}", "\u{2b1c}"] | 
            ["\u{2b55}", "\u{2b1c}", "\u{2b55}"] | 
            ["\u{2b1c}", "\u{2b55}", "\u{2b55}"] => {
                let index = diag1.iter().position(|&r| r == "\u{2b1c}").unwrap()*4;
                let _ = std::mem::replace(&mut ownvec[index], cpuchoice);
                return statecheck(ownvec, cpuchoice, playerchoice);
             }
             _ => {}
        }
        match diag2[..]{
            ["\u{2b55}", "\u{2b55}", "\u{2b1c}"] | 
            ["\u{2b55}", "\u{2b1c}", "\u{2b55}"] | 
            ["\u{2b1c}", "\u{2b55}", "\u{2b55}"] => {
                let index = diag2.iter().position(|&r| r == "\u{2b1c}").unwrap()*2+2;
                let _ = std::mem::replace(&mut ownvec[index], cpuchoice);
                return statecheck(ownvec, cpuchoice, playerchoice);
             }
             _ => {}
        }
        match vert1[..]{
            ["\u{2b55}", "\u{2b55}", "\u{2b1c}"] | 
            ["\u{2b55}", "\u{2b1c}", "\u{2b55}"] | 
            ["\u{2b1c}", "\u{2b55}", "\u{2b55}"] => {
                let index = vert1.iter().position(|&r| r == "\u{2b1c}").unwrap()*3;
                let _ = std::mem::replace(&mut ownvec[index], cpuchoice);
                return statecheck(ownvec, cpuchoice, playerchoice);
             }
             _ => {}
        }
        match vert2[..]{
            ["\u{2b55}", "\u{2b55}", "\u{2b1c}"] | 
            ["\u{2b55}", "\u{2b1c}", "\u{2b55}"] | 
            ["\u{2b1c}", "\u{2b55}", "\u{2b55}"] => {
                let index = vert2.iter().position(|&r| r == "\u{2b1c}").unwrap()*3+1;
                let _ = std::mem::replace(&mut ownvec[index], cpuchoice);
                return statecheck(ownvec, cpuchoice, playerchoice);
             }
             _ => {}
        }
        match vert3[..]{
            ["\u{2b55}", "\u{2b55}", "\u{2b1c}"] | 
            ["\u{2b55}", "\u{2b1c}", "\u{2b55}"] | 
            ["\u{2b1c}", "\u{2b55}", "\u{2b55}"] => {
                let index = vert3.iter().position(|&r| r == "\u{2b1c}").unwrap()*3+2;
                let _ = std::mem::replace(&mut ownvec[index], cpuchoice);
                return statecheck(ownvec, cpuchoice, playerchoice);
             }
             _ => {}
        }
    }
    
    println!("my greatest embarassment");
    let mut rng: ThreadRng = rand::thread_rng();
    let mut ran: usize = rng.gen_range(0..9);

    if ownvec.contains(&"\u{2b1c}") == false{
        return(ownvec, Gamestate::Draw)
    }
    while turnsvec[ran] != "\u{2b1c}"{
        ran = rng.gen_range(0..9);
    }

    let _ = std::mem::replace(&mut ownvec[ran], cpuchoice);

    return statecheck(ownvec, cpuchoice, playerchoice);
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let token = std::env::var("API_TOKEN").expect("TOKEN must be set");
    let api = Api::new(token);

    // Fetch new updates via long poll method
    let mut curvec = vec!["\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}"];
    let mut stream = api.stream();
    let mut stat = Gamestate::Nowin;
    let mut iteration = 0;
    let (mut cpuchoice, mut playerchoice) = ("", "");
    let mut turnsvec = vec!["\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}"];

    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref entities, ref data } = message.kind {
                if iteration == 0{
                    stat = Gamestate::Nowin;
                    curvec = vec!["\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}"];
                    stream = api.stream();
                    (cpuchoice, playerchoice) = ("", "");
                    turnsvec = vec!["\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}", "\u{2b1c}"];
                    let ent: Option<MessageEntity> = entities.get(0).cloned();
                    if let Some(entity) = ent{
                        if MessageEntityKind::BotCommand == entity.kind && data.contains("/start"){
                            api.send(message.text_reply("Hello, would you like to play a game of XO? Type 'Yes' if you do, and 'No' if you don't. ")).await?;
                            iteration = 1;
                        }
                    }
                }
                else if iteration == 1 {
                    match data.as_str(){
                        "Yes" => {
                            api.send(message.text_reply("Very well, do you want to play as X's or as O's? Type in the letter that you want to pick.")).await?;
                            iteration = 2;
                        }
                        "No" => {
                            api.send(message.text_reply("Okay, type in the command '/start' whenever you want to play the game")).await?;
                            iteration = 0;
                        }
                        _ => {}
                    }
                }
                else if iteration == 2{
                    match data.as_str(){
                        "X" => {
                            cpuchoice = "\u{2b55}";
                            playerchoice = "\u{274c}";
                            api.send(message.text_reply
                                ("Okay then, this is the field now, choose the number of a cell that you want to pick: \n \n \u{2b1c}\u{2b1c}\u{2b1c} \n \u{2b1c}\u{2b1c}\u{2b1c}  \n \u{2b1c}\u{2b1c}\u{2b1c} ")).await?;
                            iteration = 3
                        }
                        "O" => {
                            cpuchoice = "\u{274c}";
                            playerchoice = "\u{2b55}";
                            (curvec, _) = xoalg(&turnsvec, cpuchoice, playerchoice);
                            api.send(message.text_reply
                                (format!("Okay then, my turn is first, here is the field, choose the number of the cell you want to pick \n \n  {}{}{} \n  {}{}{} \n  {}{}{} ", 
                                curvec[0], curvec[1], curvec[2], curvec[3], curvec[4], curvec[5], curvec[6], curvec[7], curvec[8]))).await?;
                            iteration = 3
                        }
                        _ => {}
                    }
                }
                else if iteration == 3{
                    let resdata = data.parse::<usize>();
                    let int_data = match resdata{
                        Ok(x) => x,
                        Err(_) => 0
                    };
                    if int_data <= 0 || int_data >= 10 {
                        api.send(message.text_reply("Please, enter a value from 1 to 9")).await?;
                    } else if curvec[int_data-1] == "\u{2b1c}" {
                        let _ = std::mem::replace(&mut curvec[int_data-1], playerchoice);
                        (curvec, stat) = xoalg(&curvec, cpuchoice, playerchoice);
                        if matches!(stat, Gamestate::Nowin){

                            api.send(message.text_reply(format!("\n {}{}{} \n {}{}{} \n {}{}{}", 
                                curvec[0], curvec[1], curvec[2], curvec[3], curvec[4], curvec[5], curvec[6], curvec[7], curvec[8]))).await?;

                        }else if matches!(stat, Gamestate::Playerwin){

                            api.send(message.text_reply(format!("\n {}{}{} \n {}{}{} \n {}{}{}", 
                                curvec[0], curvec[1], curvec[2], curvec[3], curvec[4], curvec[5], curvec[6], curvec[7], curvec[8]))).await?;
                            api.send(message.text_reply("You win! Type in the command /start to play again")).await?;

                            iteration = 0;
                        } else if matches!(stat, Gamestate::CPUwin){

                            api.send(message.text_reply(format!("\n {}{}{} \n {}{}{} \n {}{}{}", 
                                curvec[0], curvec[1], curvec[2], curvec[3], curvec[4], curvec[5], curvec[6], curvec[7], curvec[8]))).await?;
                            api.send(message.text_reply("You lose :( Type in the command /start to play again")).await?;

                            iteration = 0;

                        } else if matches!(stat, Gamestate::Draw){

                            api.send(message.text_reply(format!("\n {}{}{} \n {}{}{} \n {}{}{}", 
                                curvec[0], curvec[1], curvec[2], curvec[3], curvec[4], curvec[5], curvec[6], curvec[7], curvec[8]))).await?;
                            api.send(message.text_reply("A Draw! Type in the command /start to play again")).await?;

                            iteration = 0;
                        };
                    } else {
                        api.send(message.text_reply("This square was taken, choose another one")).await?;
                    }
                }
            }
        }
    } Ok(())
}