use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_spray_can_sparkles (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M96 32C96 14.33 110.3 0 128 0H192C209.7 0 224 14.33 224 32V128H96V32zM224 160C277 160 320 202.1 320 256V464C320 490.5 298.5 512 272 512H48C21.49 512 0 490.5 0 464V256C0 202.1 42.98 160 96 160H224zM160 416C204.2 416 240 380.2 240 336C240 291.8 204.2 256 160 256C115.8 256 80 291.8 80 336C80 380.2 115.8 416 160 416zM384 48C384 49.36 383 50.97 381.8 51.58L352 64L339.6 93.78C338.1 95 337.4 96 336 96C334.6 96 333 95 332.4 93.78L320 64L290.2 51.58C288.1 50.97 288 49.36 288 48C288 46.62 288.1 45.03 290.2 44.42L320 32L332.4 2.219C333 1 334.6 0 336 0C337.4 0 338.1 1 339.6 2.219L352 32L381.8 44.42C383 45.03 384 46.62 384 48zM460.4 93.78L448 64L418.2 51.58C416.1 50.97 416 49.36 416 48C416 46.62 416.1 45.03 418.2 44.42L448 32L460.4 2.219C461 1 462.6 0 464 0C465.4 0 466.1 1 467.6 2.219L480 32L509.8 44.42C511 45.03 512 46.62 512 48C512 49.36 511 50.97 509.8 51.58L480 64L467.6 93.78C466.1 95 465.4 96 464 96C462.6 96 461 95 460.4 93.78zM467.6 194.2L480 224L509.8 236.4C511 237 512 238.6 512 240C512 241.4 511 242.1 509.8 243.6L480 256L467.6 285.8C466.1 287 465.4 288 464 288C462.6 288 461 287 460.4 285.8L448 256L418.2 243.6C416.1 242.1 416 241.4 416 240C416 238.6 416.1 237 418.2 236.4L448 224L460.4 194.2C461 193 462.6 192 464 192C465.4 192 466.1 193 467.6 194.2zM448 144C448 145.4 447 146.1 445.8 147.6L416 160L403.6 189.8C402.1 191 401.4 192 400 192C398.6 192 397 191 396.4 189.8L384 160L354.2 147.6C352.1 146.1 352 145.4 352 144C352 142.6 352.1 141 354.2 140.4L384 128L396.4 98.22C397 97 398.6 96 400 96C401.4 96 402.1 97 403.6 98.22L416 128L445.8 140.4C447 141 448 142.6 448 144z" /></ svg > } }