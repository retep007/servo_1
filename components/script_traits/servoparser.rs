trait ServoParser {
    fn parse_html_script_input(document: &Document, url: ServoUrl, type_: &str);
}