use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_staff_aesculapius (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 384 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M222.5 48H288C341 48 384 90.98 384 144C384 197 341 240 288 240H248V160H288C296.8 160 304 152.8 304 144C304 135.2 296.8 128 288 128H220L215.5 272H256C309 272 352 314.1 352 368C352 421 309 464 256 464H240V384H256C264.8 384 272 376.8 272 368C272 359.2 264.8 352 256 352H212.1L208.5 496C208.2 504.9 200.9 512 191.1 512C183.1 512 175.8 504.9 175.5 496L174.5 464H135.1C113.9 464 95.1 446.1 95.1 424C95.1 401.9 113.9 384 135.1 384H171.1L170.1 352H151.1C98.98 352 55.1 309 55.1 256C55.1 208.4 90.6 168.9 135.1 161.3V256C135.1 264.8 143.2 272 151.1 272H168.5L164 128H122.6C113.6 146.9 94.34 160 72 160H56C25.07 160 0 134.9 0 104C0 73.07 25.07 48 56 48H161.5L160.1 31.98C160.1 31.33 160.1 30.69 160.1 30.05C161.5 13.43 175.1 0 192 0C208.9 0 222.5 13.43 223 30.05C223 30.69 223 31.33 223 31.98L222.5 48zM79.1 96C79.1 87.16 72.84 80 63.1 80C55.16 80 47.1 87.16 47.1 96C47.1 104.8 55.16 112 63.1 112C72.84 112 79.1 104.8 79.1 96z" /></ svg > } }