use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_border_none (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 448 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M64 448C64 465.7 49.67 480 32 480C14.33 480 0 465.7 0 448C0 430.3 14.33 416 32 416C49.67 416 64 430.3 64 448zM128 480C110.3 480 96 465.7 96 448C96 430.3 110.3 416 128 416C145.7 416 160 430.3 160 448C160 465.7 145.7 480 128 480zM128 96C110.3 96 96 81.67 96 64C96 46.33 110.3 32 128 32C145.7 32 160 46.33 160 64C160 81.67 145.7 96 128 96zM160 256C160 273.7 145.7 288 128 288C110.3 288 96 273.7 96 256C96 238.3 110.3 224 128 224C145.7 224 160 238.3 160 256zM320 480C302.3 480 288 465.7 288 448C288 430.3 302.3 416 320 416C337.7 416 352 430.3 352 448C352 465.7 337.7 480 320 480zM352 64C352 81.67 337.7 96 320 96C302.3 96 288 81.67 288 64C288 46.33 302.3 32 320 32C337.7 32 352 46.33 352 64zM320 288C302.3 288 288 273.7 288 256C288 238.3 302.3 224 320 224C337.7 224 352 238.3 352 256C352 273.7 337.7 288 320 288zM256 448C256 465.7 241.7 480 224 480C206.3 480 192 465.7 192 448C192 430.3 206.3 416 224 416C241.7 416 256 430.3 256 448zM224 96C206.3 96 192 81.67 192 64C192 46.33 206.3 32 224 32C241.7 32 256 46.33 256 64C256 81.67 241.7 96 224 96zM256 256C256 273.7 241.7 288 224 288C206.3 288 192 273.7 192 256C192 238.3 206.3 224 224 224C241.7 224 256 238.3 256 256zM416 480C398.3 480 384 465.7 384 448C384 430.3 398.3 416 416 416C433.7 416 448 430.3 448 448C448 465.7 433.7 480 416 480zM416 96C398.3 96 384 81.67 384 64C384 46.33 398.3 32 416 32C433.7 32 448 46.33 448 64C448 81.67 433.7 96 416 96zM64 64C64 81.67 49.67 96 32 96C14.33 96 0 81.67 0 64C0 46.33 14.33 32 32 32C49.67 32 64 46.33 64 64zM416 288C398.3 288 384 273.7 384 256C384 238.3 398.3 224 416 224C433.7 224 448 238.3 448 256C448 273.7 433.7 288 416 288zM64 256C64 273.7 49.67 288 32 288C14.33 288 0 273.7 0 256C0 238.3 14.33 224 32 224C49.67 224 64 238.3 64 256zM224 384C206.3 384 192 369.7 192 352C192 334.3 206.3 320 224 320C241.7 320 256 334.3 256 352C256 369.7 241.7 384 224 384zM448 352C448 369.7 433.7 384 416 384C398.3 384 384 369.7 384 352C384 334.3 398.3 320 416 320C433.7 320 448 334.3 448 352zM32 384C14.33 384 0 369.7 0 352C0 334.3 14.33 320 32 320C49.67 320 64 334.3 64 352C64 369.7 49.67 384 32 384zM448 160C448 177.7 433.7 192 416 192C398.3 192 384 177.7 384 160C384 142.3 398.3 128 416 128C433.7 128 448 142.3 448 160zM32 192C14.33 192 0 177.7 0 160C0 142.3 14.33 128 32 128C49.67 128 64 142.3 64 160C64 177.7 49.67 192 32 192zM256 160C256 177.7 241.7 192 224 192C206.3 192 192 177.7 192 160C192 142.3 206.3 128 224 128C241.7 128 256 142.3 256 160z" /></ svg > } }