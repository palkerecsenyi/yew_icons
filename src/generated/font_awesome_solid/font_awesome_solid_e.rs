use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_e (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 320 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M320 448c0 17.67-14.33 32-32 32H32c-17.67 0-32-14.33-32-32v-384C0 46.34 14.33 32.01 32 32.01h256c17.67 0 32 14.33 32 32s-14.33 32-32 32H64v128h160c17.67 0 32 14.32 32 31.99s-14.33 32.01-32 32.01H64v128h224C305.7 416 320 430.3 320 448z" /></ svg > } }