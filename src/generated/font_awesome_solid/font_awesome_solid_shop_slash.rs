use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_shop_slash (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 640 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M74.13 32.8L81.75 21.38C90.65 8.022 105.6 .001 121.7 .001H518.3C534.4 .001 549.3 8.022 558.2 21.38L633.8 134.7C637.8 140.8 640 147.9 640 155.2C640 175.5 623.5 192 603.2 192H277.3L320 225.5V224H384V275.7L512 375.1V224H576V426.2L630.8 469.1C641.2 477.3 643.1 492.4 634.9 502.8C626.7 513.2 611.6 515.1 601.2 506.9L9.196 42.89C-1.236 34.71-3.065 19.63 5.112 9.196C13.29-1.236 28.37-3.065 38.81 5.112L74.13 32.8zM0 155.2C0 147.9 2.153 140.8 6.188 134.7L20.98 112.5L121.8 192H36.84C16.5 192 .0003 175.5 .0003 155.2H0zM320 384V348.1L384 398.5V464C384 490.5 362.5 512 336 512H112C85.49 512 64 490.5 64 464V224H128V384H320z" /></ svg > } }