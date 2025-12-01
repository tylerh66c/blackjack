/*
By: <tyler>
Date: 2025-11-14
Program Details: <black jack>
*/

mod modules;
use crate::miniquad::date;
use crate::modules::label::Label;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use macroquad::prelude::*;
use crate::modules::preload_image::TextureManager;
use crate::modules::preload_image::LoadingScreenOptions;
/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "Black Jack".to_string(),
        window_width: 1124,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(date::now() as u64);
    let tm = TextureManager::new();
   let loading_options = LoadingScreenOptions {
       title: Some("Black Jack".to_string()),
       background_color: DARKGREEN,
       bar_fill_color: GOLD,
       // Use default values for other options
       ..Default::default()
   };
   tm.preload_with_loading_screen(&["assets/Two-of-clubs.png","assets/Two-of-hearts.png", "assets/Two-of-spades.png", "assets/Two-of-diamonds.png","assets/Three-of-hearts.png", "assets/Three-of-diamonds.png", "assets/Three-of-clubs.png", "assets/Three-of-spades.png", "assets/Four-of-hearts.png", "assets/Four-of-diamonds.png","assets/Four-of-clubs.png","assets/Four-of-spades.png","assets/Five-of-hearts.png","assets/Five-of-diamonds.png","assets/Five-of-clubs.png","assets/Five-of-spades.png","assets/Six-of-hearts.png","assets/Six-of-diamonds.png","assets/Six-of-spades.png", "assets/Six-of-clubs.png","assets/Seven-of-hearts.png","assets/Seven-of-diamonds.png", "assets/Seven-of-clubs.png", "assets/Seven-of-spades.png", "assets/Eight-of-hearts.png", "assets/Eight-of-diamonds.png", "assets/Eight-of-spades.png", "assets/Eight-of-clubs.png", "assets/Nine-of-hearts.png", "assets/Nine-of-diamonds.png", "assets/Nine-of-clubs.png", "assets/Nine-of-spades.png", "assets/Ten-of-hearts.png", "assets/Ten-of-diamonds.png", "assets/Ten-of-spades.png", "assets/Ten-of-clubs.png", "assets/Ace-of-hearts.png", "assets/Ace-of-diamonds.png", "assets/Ace-of-spades.png", "assets/Ace-of-clubs.png", "assets/Jack-of-hearts.png", "assets/Jack-of-diamonds.png", "assets/Jack-of-spades.png", "assets/Jack-of-clubs.png", "assets/Queen-of-hearts.png", "assets/Queen-of-diamonds.png", "assets/Queen-of-spades.png", "assets/Queen-of-clubs.png", "assets/King-of-hearts.png", "assets/King-of-diamonds.png", "assets/King-of-spades.png", "assets/King-of-clubs.png", "assets/Empty.png"], Some(loading_options)).await;
 
    let mut cards: Vec<&str> = vec![
        "assets/Two-of-clubs.png",
        "assets/Two-of-hearts.png",
        "assets/Two-of-spades.png",
        "assets/Two-of-diamonds.png",
        "assets/Three-of-hearts.png",
        "assets/Three-of-diamonds.png",
        "assets/Three-of-clubs.png",
        "assets/Three-of-spades.png",
        "assets/Four-of-hearts.png",
        "assets/Four-of-diamonds.png",
        "assets/Four-of-clubs.png",
        "assets/Four-of-spades.png",
        "assets/Five-of-hearts.png",
        "assets/Five-of-diamonds.png",
        "assets/Five-of-clubs.png",
        "assets/Five-of-spades.png",
        "assets/Six-of-hearts.png",
        "assets/Six-of-diamonds.png",
        "assets/Six-of-spades.png",
        "assets/Six-of-clubs.png",
        "assets/Seven-of-hearts.png",
        "assets/Seven-of-diamonds.png",
        "assets/Seven-of-clubs.png",
        "assets/Seven-of-spades.png",
        "assets/Eight-of-hearts.png",
        "assets/Eight-of-diamonds.png",
        "assets/Eight-of-spades.png",
        "assets/Eight-of-clubs.png",
        "assets/Nine-of-hearts.png",
        "assets/Nine-of-diamonds.png",
        "assets/Nine-of-clubs.png",
        "assets/Nine-of-spades.png",
        "assets/Ten-of-hearts.png",
        "assets/Ten-of-diamonds.png",
        "assets/Ten-of-spades.png",
        "assets/Ten-of-clubs.png",
        "assets/Ace-of-hearts.png",
        "assets/Ace-of-diamonds.png",
        "assets/Ace-of-spades.png",
        "assets/Ace-of-clubs.png",
        "assets/Jack-of-hearts.png",
        "assets/Jack-of-diamonds.png",
        "assets/Jack-of-spades.png",
        "assets/Jack-of-clubs.png",
        "assets/Queen-of-hearts.png",
        "assets/Queen-of-diamonds.png",
        "assets/Queen-of-spades.png",
        "assets/Queen-of-clubs.png",
        "assets/King-of-hearts.png",
        "assets/King-of-diamonds.png",
        "assets/King-of-spades.png",
        "assets/King-of-clubs.png",
    ];
    cards.push("assets/empty.png");
    let mut scores = vec![
        2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 8, 9, 9, 9, 9, 10, 10, 10, 10, 11, 11, 11, 11, 10, 10, 10,
        10, 10, 10, 10, 10, 10, 10, 10, 10,
    ];
    scores.push(0);

    let mut first_card = StillImage::new("assets/Empty.png", 110.0, 160.0, 100.0, 500.0, true, 1.0).await;
    let mut second_card = StillImage::new("assets/Empty.png", 110.0, 160.0, 225.0, 500.0, true, 1.0).await;
    let mut dealer_card1 = StillImage::new("assets/Empty.png", 110.0, 160.0, 100.0, 100.0, true, 1.0).await;
    let mut dealer_card2 = StillImage::new("assets/Empty.png", 110.0, 160.0, 225.0, 100.0, true, 1.0).await;
    let mut dealer_card3 = StillImage::new("assets/Empty.png", 110.0, 160.0, 350.0, 100.0, true, 1.0).await;
    let mut dealer_card4 = StillImage::new("assets/Empty.png", 110.0, 160.0, 475.0, 100.0, true, 1.0).await;
    let mut dealer_card5 = StillImage::new("assets/Empty.png", 110.0, 160.0, 600.0, 100.0, true, 1.0).await;
    let mut fourth_card = StillImage::new("assets/Empty.png", 110.0, 160.0, 475.0, 500.0, true, 1.0).await;
    let mut third_card = StillImage::new("assets/Empty.png", 110.0, 160.0, 350.0, 500.0, true, 1.0).await;
    let btn_exit = TextButton::new(780.0, 0.0, 200.0, 65.0, "Exit", BLACK, DARKGRAY, 35);
    let mut fifth_card = StillImage::new("assets/Empty.png", 110.0, 160.0, 600.0, 500.0, true, 1.0).await;
    let mut btn_deal = TextButton::new(100.0, 350.0, 200.0, 65.0, "Deal", BLACK, DARKGRAY, 35);
    let mut btn_hit = TextButton::new(330.0, 350.0, 170.0, 65.0, "Hit", BLACK, DARKGRAY, 35);
    btn_hit.enabled = false;
    let mut btn_stand = TextButton::new(530.0, 350.0, 170.0, 65.0, "Stand", BLACK, DARKGRAY, 35);
    btn_stand.enabled = false;
    let mut btn_replay = TextButton::new(750.0, 350.0, 200.0, 65.0, "Play Again", BLACK, DARKGRAY, 30);
    let lbl_dealerhand = Label::new("Dealer's Hand", 70.0, 80.0, 30);
    let mut lbl_winner = Label::new("", 525.0, 60.0, 40);
    let lbl_playerhand = Label::new("Your Hand", 70.0, 475.0, 30);
    let mut lbl_playerscore = Label::new("", 300.0, 475.0, 40);
    let mut lbl_dealerscore = Label::new("", 300.0, 80.0, 40);
    let lbl_playerwins: Label = Label::new("Your Wins:", 750.0, 100.0, 30);
    let lbl_dealerwins: Label = Label::new("Dealer Wins:", 725.0, 140.0, 30);
    let mut lbl_playercounter: Label = Label::new("0", 890.0, 100.0, 30);
    let mut lbl_dealercounter: Label = Label::new("0", 890.0, 140.0, 30);
    let mut numofhits = 0;
    let mut playertotal = 0;
    let mut dealertotal = 0;

    loop {
        clear_background(DARKGREEN);
        let random_card_1 = rand::gen_range(1, 52);
        let random_card_2 = rand::gen_range(1, 52);
                if playertotal > 20 {
                    btn_hit.enabled = false;
                }
        if btn_exit.click() {
            break;
        }
        if btn_deal.click() {
            first_card.set_texture(cards[random_card_1]).await;
            second_card.set_texture(cards[random_card_2]).await;
            playertotal = scores[random_card_1] + scores[random_card_2];
            lbl_playerscore.set_text(format!("{}", playertotal));
            if playertotal > 20 {
                btn_hit.enabled = false;
            }
            let random_dealer_1 = rand::gen_range(1, 52);
            dealer_card1.set_texture(cards[random_dealer_1]).await;
            dealertotal = scores[random_dealer_1];
            lbl_dealerscore.set_text(format!("{}", dealertotal));
            btn_deal.enabled = false;
            btn_hit.enabled = true;
            btn_stand.enabled = true;
            btn_replay.enabled = false;
        }
        if btn_hit.click() {
            numofhits += 1;
             let random_card_3 = rand::gen_range(1, 52);

            if numofhits == 1 {
                third_card.set_texture(cards[random_card_3]).await;
                playertotal += scores[random_card_3];
                if playertotal > 22 {
                    btn_hit.enabled = false;
                }

                lbl_playerscore.set_text(format!("{}", playertotal));

            } else if numofhits==2 {
                 fourth_card.set_texture(cards[random_card_3]).await;
                playertotal += scores[random_card_3];
                lbl_playerscore.set_text(format!("{}", playertotal));
                if playertotal > 20 {
                    btn_hit.enabled = false;
                }
            } else if numofhits==3 {
                btn_hit.enabled = false;
                 fifth_card.set_texture(cards[random_card_3]).await;
                playertotal += scores[random_card_3];
                lbl_playerscore.set_text(format!("{}", playertotal));
                if playertotal > 20 {
                    btn_hit.enabled = false;
                }
            }
        }
        if btn_stand.click() {
            let random_dealer_2 = rand::gen_range(1, 52);
            let random_dealer_3 = rand::gen_range(1, 52);
            let random_dealer_4 = rand::gen_range(1, 52);
            let random_dealer_5 = rand::gen_range(1, 52);
            dealer_card2.set_texture(cards[random_dealer_2]).await;
            dealertotal += scores[random_dealer_2];
            lbl_dealerscore.set_text(format!("{}", dealertotal));
            if dealertotal < 16 {
            dealer_card3.set_texture(cards[random_dealer_3]).await;
            dealertotal += scores[random_dealer_3];
            lbl_dealerscore.set_text(format!("{}", dealertotal));
            }
            if dealertotal < 16 {
            dealer_card4.set_texture(cards[random_dealer_4]).await;
            dealertotal += scores[random_dealer_4];
            lbl_dealerscore.set_text(format!("{}", dealertotal));
            }
            if dealertotal < 16 {
            dealer_card5.set_texture(cards[random_dealer_5]).await;
            dealertotal += scores[random_dealer_5];
            lbl_dealerscore.set_text(format!("{}", dealertotal));
            }

            if playertotal > 21 && dealertotal < 22 {
                lbl_winner.set_text("Dealer Wins!");
                lbl_dealercounter.set_text(format!("{}", lbl_dealercounter.get_text().parse::<i32>().unwrap() + 1));
            } else if dealertotal > 21 && playertotal < 22 {
                lbl_winner.set_text("You Win!");
                lbl_playercounter.set_text(format!("{}", lbl_playercounter.get_text().parse::<i32>().unwrap() + 1));
            } else if dealertotal > playertotal && dealertotal < 22 {
                lbl_winner.set_text("Dealer Wins!");
                lbl_dealercounter.set_text(format!("{}", lbl_dealercounter.get_text().parse::<i32>().unwrap() + 1));
            } else if dealertotal < playertotal && playertotal < 22 {
                lbl_winner.set_text("You Win!");
                lbl_playercounter.set_text(format!("{}", lbl_playercounter.get_text().parse::<i32>().unwrap() + 1));
            } else if dealertotal > 21 && playertotal > 21 {
                lbl_winner.set_text("No Winner!");
            } else {
                lbl_winner.set_text("Draw!");
            }

            btn_hit.enabled = false;
            btn_stand.enabled = false;
            btn_replay.enabled = true;
        }
        if btn_replay.click() {
            first_card.set_texture("assets/Empty.png").await;
            second_card.set_texture("assets/Empty.png").await;
            dealer_card1.set_texture("assets/Empty.png").await;
            dealer_card2.set_texture("assets/Empty.png").await;
            dealer_card3.set_texture("assets/Empty.png").await;
            third_card.set_texture("assets/Empty.png").await;
            fourth_card.set_texture("assets/Empty.png").await;
            fifth_card.set_texture("assets/Empty.png").await;
            btn_deal.enabled = true;
            btn_hit.enabled = false;
            btn_stand.enabled = false;
            lbl_playerscore.set_text("");
            lbl_dealerscore.set_text("");
            numofhits = 0;
            lbl_winner.set_text("");
        }
        first_card.draw();
        second_card.draw();
        third_card.draw();
        fourth_card.draw();
        dealer_card1.draw();
        dealer_card2.draw();
        lbl_dealerhand.draw();
        lbl_playerhand.draw();
        lbl_playerscore.draw();
        lbl_dealerscore.draw();
        dealer_card3.draw();
        fifth_card.draw();
        lbl_winner.draw();
        lbl_playerwins.draw();
        lbl_dealerwins.draw();
        lbl_dealercounter.draw();
        lbl_playercounter.draw();
        next_frame().await;
    }
}