use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_plug_circle_plus (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 576 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M96 0C113.7 0 128 14.33 128 32V128H64V32C64 14.33 78.33 0 96 0zM288 0C305.7 0 320 14.33 320 32V128H256V32C256 14.33 270.3 0 288 0zM352 160C369.7 160 384 174.3 384 192C384 194.3 383.7 196.6 383.3 198.8C309.8 219.1 256 287.7 256 368C256 379.4 257.1 390.5 259.1 401.3C248.1 406.4 236.3 410.3 224 412.8V512H160V412.8C86.97 397.1 32 333.4 32 256V224C14.33 224 0 209.7 0 192C0 174.3 14.33 160 32 160H352zM288 368C288 288.5 352.5 224 432 224C511.5 224 576 288.5 576 368C576 447.5 511.5 512 432 512C352.5 512 288 447.5 288 368zM448 303.1C448 295.2 440.8 287.1 432 287.1C423.2 287.1 416 295.2 416 303.1V351.1H368C359.2 351.1 352 359.2 352 367.1C352 376.8 359.2 383.1 368 383.1H416V431.1C416 440.8 423.2 447.1 432 447.1C440.8 447.1 448 440.8 448 431.1V383.1H496C504.8 383.1 512 376.8 512 367.1C512 359.2 504.8 351.1 496 351.1H448V303.1z" /></ svg > } }