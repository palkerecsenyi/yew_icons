use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_circle_nodes (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M380.6 365.6C401.1 379.9 416 404.3 416 432C416 476.2 380.2 512 336 512C291.8 512 256 476.2 256 432C256 423.6 257.3 415.4 259.7 407.8L114.1 280.4C103.8 285.3 92.21 288 80 288C35.82 288 0 252.2 0 208C0 163.8 35.82 128 80 128C101.9 128 121.7 136.8 136.2 151.1L320 77.52C321.3 34.48 356.6 0 400 0C444.2 0 480 35.82 480 80C480 117.9 453.7 149.6 418.4 157.9L380.6 365.6zM156.3 232.2L301.9 359.6C306.9 357.3 312.1 355.4 317.6 354.1L355.4 146.4C351.2 143.6 347.4 140.4 343.8 136.9L159.1 210.5C159.7 218 158.5 225.3 156.3 232.2V232.2z" /></ svg > } }