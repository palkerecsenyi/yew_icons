use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_ae (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } id = "flag-icons-ae" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill = "#00732f" d = "M0 0h512v170.7H0z" /> < path fill = "#fff" d = "M0 170.7h512v170.6H0z" /> < path d = "M0 341.3h512V512H0z" /> < path fill = "red" d = "M0 0h180v512H0z" /> </ svg > } }