use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_gb_eng (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } id = "flag-icons-gb-eng" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill = "#fff" d = "M0 0h512v512H0z" /> < path fill = "#ce1124" d = "M215 0h82v512h-82z" /> < path fill = "#ce1124" d = "M0 215h512v82H0z" /> </ svg > } }