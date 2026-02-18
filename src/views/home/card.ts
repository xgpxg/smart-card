class Card {
    // 卡片标题
    title: string;
    // 卡片内容
    content: string;
    // 卡片配置项
    options?: CardOptions;

    constructor(title: string, content: string, options?: CardOptions) {
        this.title = title;
        this.content = content;
        this.options = options;
    }
}

class CardOptions{

}


export {
    Card
}