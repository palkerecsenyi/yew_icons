use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_hat_cowboy_side (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 640 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M260.8 260C232.1 237.1 198.8 225 164.4 225c-77.38 0-142.9 62.75-163 156c-3.5 16.62-.375 33.88 8.625 47.38c8.75 13.12 21.88 20.62 35.88 20.62H592c-103.2 0-155-37.13-233.2-104.5L260.8 260zM495.5 241.8l-27.13-156.5c-2.875-17.25-12.75-32.5-27.12-42.25c-14.37-9.75-32.24-13.3-49.24-9.675L200.9 74.02C173.7 79.77 153.5 102.3 150.5 129.8L143.6 195c6.875-.875 13.62-2 20.75-2c41.87 0 82 14.5 117.4 42.88l98 84.37c71 61.25 115.1 96.75 212.2 96.75c26.5 0 48-21.5 48-48C640 343.6 610.4 249.6 495.5 241.8z" /></ svg > } }