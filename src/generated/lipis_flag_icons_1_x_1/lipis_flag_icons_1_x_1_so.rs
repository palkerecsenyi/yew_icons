use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_so (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } id = "flag-icons-so" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "so-a" > < path fill - opacity = ".7" d = "M177.2 0h708.6v708.7H177.2z" /> </ clippath > </ defs > < g fill - rule = "evenodd" transform = "translate(-128) scale(.72249)" > < path fill = "#40a6ff" d = "M0 0h1063v708.7H0z" /> < path fill = "#fff" d = "m643 527.6-114.3-74-113.6 74.8 42.3-121.5-113.5-75 140.4-1 43.5-121.1 44.5 120.8 140.3.1-112.9 75.7L643 527.6z" /> </ g > </ svg > } }