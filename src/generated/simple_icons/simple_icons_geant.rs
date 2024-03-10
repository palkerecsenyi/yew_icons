use crate :: IconProps ; # [inline (never)] pub fn simple_icons_geant (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M20.31 14.857c-.027-.014-.04-.014-.027-.07.014-.068.041-.152.055-.207.028-.097.083-.153.25-.166.138-.014.305-.014.402-.056-.125.36-.5.263-.68.5zm3.314.57c-.07.304-.305.525-.513.512-.139-.014-.18-.153-.139-.305.056-.25.278-.513.5-.5.124.014.194.125.152.292m-.083-.486c-.458-.027-.818.305-.874.652-.332.347-.47.305-.443.18.041-.152.25-.54.263-.693 0-.083-.04-.14-.152-.152-.18-.014-.486.18-.777.415-.055.042-.083.014-.07-.027.042-.11.07-.167.112-.278.014-.027.014-.04-.014-.055a.314.314 0 0 0-.166-.083c-.056 0-.097.014-.125.07-.25.22-.735.886-.9.873-.029 0-.057-.014-.057-.056 0-.097.32-.75.32-.818 0-.04-.029-.055-.056-.055-.07 0-.416.11-.444.138-.014.014-.014.028-.028.056.042 0 .07.014.056.083-.042.152-.278.638-.278.735 0 .097.042.152.153.152.291.014.582-.194.97-.638-.097.195-.166.375-.18.5-.014.124.166.138.264.124.041 0 .041 0 .055-.04.042-.112.166-.334.444-.584.124-.11.29-.235.36-.22.056 0 .056.054.056.082-.028.125-.125.32-.195.5-.055.165.014.276.153.276.18.014.43-.097.68-.332.027.194.166.346.415.36.458.028.888-.305.915-.763.014-.22-.194-.388-.457-.402m-3.799.236c0 .014.014.014.028 0 .07-.028.152-.083.152-.14 0-.068-.083-.11-.346-.123-.402-.028-.75.11-.75.332 0 .278.542.32.542.555 0 .11-.125.208-.36.208-.167 0-.306-.056-.306-.18 0-.083.07-.167.32-.222-.029-.042-.056-.042-.098-.042-.29.042-.458.153-.458.29-.013.154.111.279.583.293.527.013.748-.222.748-.43 0-.236-.332-.36-.485-.472-.04-.027-.07-.07-.07-.097 0-.055.07-.124.264-.124.083 0 .18.027.208.055.042.014.042.055.028.097zm-2.44.68c-.08 0-.122-.056-.122-.125.014-.32.5-.707.97-.638-.236.54-.596.776-.845.762m1.29-.804c.024-.04.01-.083-.03-.097-.07-.014-.168-.027-.25-.04-.913-.057-1.551.47-1.58.886-.012.166.085.277.32.29.25 0 .569-.138.776-.29.055-.042.055-.028.055.014-.04.138-.028.263.11.277.152.014.346-.07.415-.097 0 0 .014-.028.014-.055-.083 0-.11-.084-.097-.167.014-.208.18-.568.263-.72m-1.383-.278c0 .028.014.028.027.028.208 0 .36-.14.36-.222 0-.083-.096-.208-.5-.208-1.177-.014-1.953.513-2.022 1.095-.084.777 1.053 1.18 2.024.943.222-.055.346-.125.346-.166v-.084a1.22 1.22 0 0 1-.208.07c-.79.235-1.733-.083-1.65-.804.056-.513.722-.93 1.429-.915a.27.27 0 0 1 .11.013c.098.014.14.056.14.11.013.07 0 .098-.056.14M7.28 8.909c.333-.152 1.969-1.123 2.024-1.15.07-.041.347-.07.5.208.152.277.152.554-.237.707-.388.138-2.024.388-2.287.235m2.274 2.052c-.056-.527-.36-.86-.832-.86s-.777.32-.832.86zm1.164 1.94c-.86.528-1.58.528-1.885.528-1.345 0-2.22-.79-2.22-2.01 0-1.414 1-2.12 2.19-2.12 1.125 0 2.04.886 1.998 2.272H7.93c.042.735.513 1.096 1.262 1.096.485 0 .79-.097 1.525-.527zm13.044-2.675h-1.179v1.692c0 .374.222.624.596.624.25 0 .375-.056.79-.222v.887c-.526.18-.83.25-1.206.25-.804 0-1.497-.374-1.497-1.456v-1.775h-.555v-.873h.583v-.86l1.29-.554v1.4h1.178zm-6.25-.36c.402-.444.801-.596 1.314-.596.638 0 1.483.388 1.483 1.54v2.55h-1.275v-2.37c0-.556-.277-.777-.638-.777-.319 0-.551.166-.884.637v2.51h-1.276V9.367h1.276zm-3.674 1.58c-.721.264-1.054.43-1.054.82 0 .22.18.414.388.414.222 0 .472-.11.666-.29zm1.733 1.637c-.32.208-.804.36-1.123.36-.402 0-.596-.125-.68-.444-.471.32-.845.444-1.261.444-.61 0-1.068-.47-1.068-.957 0-.93.777-1.178 2.385-1.69.028-.36-.18-.653-.707-.653-.486 0-.888.07-1.331.32l-.14-.736c.5-.32 1.096-.443 1.706-.443.97 0 1.789.43 1.789 1.275l.014 1.692c0 .194.055.235.18.235.083 0 .056 0 .236-.097v.666zM5.42 8.95c-.832-.43-1.178-.485-1.872-.485-1.54 0-2.08 1.22-2.08 2.08 0 1.025.79 1.913 1.97 1.913.346 0 .526-.028 1.04-.208v-.957h-.916v-.998h2.302v2.357c-.486.443-1.54.776-2.454.776-1.997 0-3.411-1.22-3.411-2.94 0-1.76 1.4-2.965 3.425-2.965.845 0 1.69.194 2.343.54z" /></ svg > } }