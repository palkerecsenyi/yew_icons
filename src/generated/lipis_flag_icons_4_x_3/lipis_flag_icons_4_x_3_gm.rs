use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_gm (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } id = "flag-icons-gm" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "gm-a" > < path fill - opacity = ".7" d = "M0-48h640v480H0z" /> </ clippath > </ defs > < g fill - rule = "evenodd" stroke - width = "1pt" transform = "translate(0 48)" > < path fill = "red" d = "M0-128h640V85.3H0z" /> < path fill = "#fff" d = "M0 85.3h640V121H0z" /> < path fill = "#009" d = "M0 120.9h640V263H0z" /> < path fill = "#fff" d = "M0 263.1h640v35.6H0z" /> < path fill = "#090" d = "M0 298.7h640V512H0z" /> </ g > </ svg > } }