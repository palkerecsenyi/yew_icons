use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_to (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } id = "flag-icons-to" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < g fill - rule = "evenodd" stroke - width = "1pt" > < path fill = "#c10000" d = "M0 0h640v480H0z" /> < path fill = "#fff" d = "M0 0h250v200.3H0z" /> < g fill = "#c10000" > < path d = "M102.8 31.2h39.9v139.6h-39.8z" /> < path d = "M192.6 81v40H53V81z" /> </ g > </ g > </ svg > } }