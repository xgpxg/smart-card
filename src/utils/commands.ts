import {convertFileSrc, invoke} from "@tauri-apps/api/core";
import {ElMessage} from "element-plus";

const IS_DEV = import.meta.env.VITE_IS_DEV

const isDev = () => {
    return IS_DEV === 1 || IS_DEV === '1'
}


class Res{
    code: number;
    msg: string;
    data: any;
    constructor(code: number, msg: string, data: any) {
        this.code = code
        this.msg = msg
        this.data = data
    }
}
const call = async (command: string, args?: any) => {
    try {
        const res: Res = await invoke(command, args)
        if (res.code !== 0) {
            let msg = res.msg
            if (isDev()) {
                msg += '，参数：'
                msg += JSON.stringify(args)
            }
            throw new Error(msg)
        }
        return res.data
    } catch (e) {
        ElMessage.error(e.message || e)
        throw e
    }

}

const convertAudioSrc = (path: string) => {
    return convertFileSrc(path)
}

export {
    call,
    convertAudioSrc
}