use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_face_kiss (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256zM287.9 300.3C274.7 292.9 257.4 288 240 288C236.4 288 233.2 290.5 232.3 293.1C231.3 297.5 232.9 301.2 236.1 302.1L236.1 302.1L236.3 303.1L236.8 303.4L237.2 303.7C238 304.1 239.2 304.9 240.6 305.8C243.4 307.6 247.2 310.3 250.8 313.4C254.6 316.5 258 319.1 260.5 323.4C262.1 326.1 264 329.8 264 332C264 334.2 262.1 337 260.5 340.6C258 344 254.6 347.5 250.8 350.6C247.2 353.7 243.4 356.4 240.6 358.2C239.2 359.1 238 359.9 237.2 360.3L236.6 360.7L236.3 360.9L236.1 361L236.1 361C233.6 362.4 232 365.1 232 368C232 370.9 233.6 373.6 236.1 374.1L236.1 374.1L236.3 375.1C236.5 375.2 236.8 375.4 237.2 375.7C238 376.1 239.2 376.9 240.6 377.8C243.4 379.6 247.2 382.3 250.8 385.4C254.6 388.5 258 391.9 260.5 395.4C262.1 398.1 264 401.8 264 403.1C264 406.2 262.1 409 260.5 412.6C258 416 254.6 419.5 250.8 422.6C247.2 425.7 243.4 428.4 240.6 430.2C239.2 431.1 238 431.9 237.2 432.3C236.8 432.6 236.5 432.8 236.3 432.9L236.1 432.1L236.1 433C232.9 434.8 231.3 438.5 232.3 442C233.2 445.5 236.4 447.1 240 447.1C257.4 447.1 274.7 443.1 287.9 435.7C294.5 432 300.4 427.5 304.7 422.3C308.9 417.2 312 410.9 312 403.1C312 397.1 308.9 390.8 304.7 385.7C300.4 380.5 294.5 375.1 287.9 372.3C285.2 370.7 282.3 369.3 279.2 367.1C282.3 366.7 285.2 365.3 287.9 363.7C294.5 360 300.4 355.5 304.7 350.3C308.9 345.2 312 338.9 312 331.1C312 325.1 308.9 318.8 304.7 313.7C300.4 308.5 294.5 303.1 287.9 300.3L287.9 300.3zM176.4 176C158.7 176 144.4 190.3 144.4 208C144.4 225.7 158.7 240 176.4 240C194 240 208.4 225.7 208.4 208C208.4 190.3 194 176 176.4 176zM336.4 240C354 240 368.4 225.7 368.4 208C368.4 190.3 354 176 336.4 176C318.7 176 304.4 190.3 304.4 208C304.4 225.7 318.7 240 336.4 240z" /></ svg > } }