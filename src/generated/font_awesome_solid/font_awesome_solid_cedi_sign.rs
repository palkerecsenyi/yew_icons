use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_cedi_sign (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 320 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M224 66.66C254.9 71.84 283.2 84.39 307.2 102.4C321.4 113 324.2 133.1 313.6 147.2C302.1 161.4 282.9 164.2 268.8 153.6C255.6 143.7 240.4 136.3 224 132V379.1C240.4 375.7 255.6 368.3 268.8 358.4C282.9 347.8 302.1 350.6 313.6 364.8C324.2 378.9 321.4 398.1 307.2 409.6C283.2 427.6 254.9 440.2 224 445.3V480C224 497.7 209.7 512 192 512C174.3 512 160 497.7 160 480V445.3C69.19 430.1 0 351.1 0 256C0 160.9 69.19 81.89 160 66.65V32C160 14.33 174.3 0 192 0C209.7 0 224 14.33 224 32V66.66zM160 132C104.8 146.2 64 196.4 64 255.1C64 315.6 104.8 365.8 160 379.1V132z" /></ svg > } }