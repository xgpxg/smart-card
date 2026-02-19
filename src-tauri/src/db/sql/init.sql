create table if not exists workspace
(
    id                bigint     not null primary key,
    file_name         varchar(500),                 -- 文件名
    raw_file_name     varchar(500),                 -- 原始文件名
    file_path         varchar(500),                 -- 文件路径
    file_size         int,                          -- 文件大小
    file_type         varchar(50),                  -- 文件类型: mp3 | ogg | wav
    audio_duration    int,                          -- 音频时长
    trans_text_status varchar(50),                  -- 音频转文本状态: NotStart | Waiting | Processing | Ok | Failed
    trans_text        text,                         -- 转换后的文本
    style_id          bigint,                       -- 样式ID
    font              varchar(500),                 -- 字体配置，JSON格式
    pagination        varchar(50),                  -- 分页方式: Auto | Single | CharCount | Delimiter
    create_user_id    bigint,                       -- 创建人ID
    update_user_id    bigint,                       -- 修改人ID
    create_time       datetime,                     -- 创建时间
    update_time       datetime,                     -- 更新时间
    remark            varchar(500),                 -- 备注
    is_delete         tinyint(1) not null default 0 -- 是否删除
)