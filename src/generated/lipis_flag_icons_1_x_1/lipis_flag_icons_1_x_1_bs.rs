use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_bs (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } id = "flag-icons-bs" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "bs-a" > < path fill - opacity = ".7" d = "M56.6 26.4H537v480.3H56.6z" /> </ clippath > </ defs > < g fill - rule = "evenodd" transform = "matrix(1.066 0 0 1.067 -60.4 -28.1)" > < path fill = "#fff" d = "M990 506.2H9.4V27.6H990z" /> < path fill = "#ffe900" d = "M990 370.6H9.4V169.2H990z" /> < path fill = "#08ced6" d = "M990 506.2H9.4V346.7H990zm0-319H9.4V27.9H990z" /> < path d = "M9 25.9c2.1 0 392.3 237 392.3 237L7.8 505.3 9 25.9z" /> </ g > </ svg > } }