//  ██████╗  █████╗ ███████╗███████╗██╗███╗   ██╗ ██████╗
//  ██╔══██╗██╔══██╗██╔════╝██╔════╝██║████╗  ██║██╔════╝
//  ██████╔╝███████║███████╗███████╗██║██╔██╗ ██║██║  ███╗
//  ██╔═══╝ ██╔══██║╚════██║╚════██║██║██║╚██╗██║██║   ██║
//  ██║     ██║  ██║███████║███████║██║██║ ╚████║╚██████╔╝
//  ╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝

#[cfg(test)]
mod passing {
    use crate::url;

    #[test]
    fn unix_file_url() {
        assert!(url::is_file_url(
            "file:///home/user/Websites/my-website/index.html"
        ));
    }

    #[test]
    fn windows_file_url() {
        assert!(url::is_file_url(
            "file:///C:/Documents%20and%20Settings/user/Websites/my-website/assets/images/logo.png"
        ));
    }

    #[test]
    fn unix_url_with_backslashes() {
        assert!(url::is_file_url(
            "file:\\\\\\home\\user\\Websites\\my-website\\index.html"
        ));
    }

    #[test]
    fn windows_file_url_with_backslashes() {
        assert!(url::is_file_url(
            "file:\\\\\\C:\\Documents%20and%20Settings\\user\\Websites\\my-website\\assets\\images\\logo.png"
        ));
    }
}

//  ███████╗ █████╗ ██╗██╗     ██╗███╗   ██╗ ██████╗
//  ██╔════╝██╔══██╗██║██║     ██║████╗  ██║██╔════╝
//  █████╗  ███████║██║██║     ██║██╔██╗ ██║██║  ███╗
//  ██╔══╝  ██╔══██║██║██║     ██║██║╚██╗██║██║   ██║
//  ██║     ██║  ██║██║███████╗██║██║ ╚████║╚██████╔╝
//  ╚═╝     ╚═╝  ╚═╝╚═╝╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝

#[cfg(test)]
mod failing {
    use crate::url;

    #[test]
    fn url_with_no_protocl() {
        assert!(!url::is_file_url("//kernel.org"));
    }

    #[test]
    fn dot_slash_filename() {
        assert!(!url::is_file_url("./index.html"));
    }

    #[test]
    fn just_filename() {
        assert!(!url::is_file_url("some-local-page.htm"));
    }

    #[test]
    fn https_ip_port_url() {
        assert!(!url::is_file_url("https://1.2.3.4:80/www/index.html"));
    }

    #[test]
    fn data_url() {
        assert!(!url::is_file_url(
            "data:text/html;base64,V2VsY29tZSBUbyBUaGUgUGFydHksIDxiPlBhbDwvYj4h"
        ));
    }

    #[test]
    fn just_word_file() {
        assert!(!url::is_file_url("file"));
    }
}
