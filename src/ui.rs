slint::slint! {
    import {ListView} from "std-widgets.slint";

    struct MainImgData {
        img: image,
        fname: string,
        size: int,
        res: [int],
        action: string,
    }

    struct ImgElemData {
        img: image,
        fname: string,
        size: int,
        res: [int],
        sim: float,
        action: string,
    }

    ActionButton := Button {
        property <string> text;
        property <color> color;
        property <string> action;
        
        text: text;
        
    }

    MainImage := Rectangle {
        property <MainImgData> data;
        width: 40%;
        Image {
            width: 80%;
            source: data.img;
        }
    }

    ImageElement := Rectangle {
        property <ImgElemData> data;
        width: 100%;
        Image {
            source: data.img;
        }
    }

    ImageElements := ListView {
        property <[ImgElemData]> imgs;

        width: 60%;
        for data in imgs : ImageElement{
            data: data;
        }
    }

    export MainWindow := Window {
        property <MainImgData> maindata;
        property <[ImgElemData]> elemsdata;
        
        default-font-size: 18px;

        HorizontalLayout {
            max-width: 800px;
            alignment: center;

            MainImage {
                data: maindata;
            }
            ImageElements {
                imgs: elemsdata;
            }
        }
    }
}
