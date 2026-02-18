const IS_DEV = import.meta.env.VITE_IS_DEV

const isDev = () => {
    return IS_DEV === 1 || IS_DEV === '1'
}

//指定一个字符串，生成固定的浅色的颜色值
const randomColor = (str: string) => {
    if (!str) {
        str = ''
    }
    let hash = 0;
    for (let i = 0; i < str.length; i++) {
        hash = str.charCodeAt(i) + ((hash << 5) - hash);
    }
    let color = '#';
    for (let i = 0; i < 3; i++) {
        // 修改：将分量范围限制在 0-127（深色系）
        const value = (hash >> (i * 8)) & 0x7F; // 原代码为 0xFF，改为 0x7F（127）
        color += ('00' + value.toString(16)).substr(-2);
    }
    return color;
}

export {
    isDev,
    randomColor
}