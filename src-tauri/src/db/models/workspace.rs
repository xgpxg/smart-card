use derive_builder::Builder;
use rbatis::executor::Executor;
use rbatis::rbdc::DateTime;
use rbatis::{crud, htmlsql};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[builder(default)]
pub struct Workspace {
    pub id: Option<i64>,
    /// 文件名
    pub file_name: Option<String>,
    /// 原始文件名
    pub raw_file_name: Option<String>,
    /// 文件路径
    pub file_path: Option<String>,
    /// 文件大小
    pub file_size: Option<u64>,
    /// 文件类型: mp3 | ogg | wav
    pub file_type: Option<String>,
    /// 音频时长，默认值为 0
    pub audio_duration: Option<i32>,
    /// 文本标题
    pub text_title: Option<String>,
    /// 音频转文本状态: NotStart | Waiting | Processing | Ok | Failed，默认值为 'NotStart'
    pub trans_text_status: Option<TransTextStatus>,
    /// 转换后的文本，可为空
    pub trans_text: Option<String>,
    /// 样式 ID
    pub style_id: Option<i64>,
    /// 字体配置，JSON 格式
    pub font: Option<Font>,
    /// 分页方式: Auto | Single | CharCount | Delimiter，默认值为 'Auto'
    pub pagination: Option<Pagination>,
    /// 创建人ID
    pub create_user_id: Option<i64>,
    /// 修改人ID
    pub update_user_id: Option<i64>,
    /// 创建时间
    pub create_time: Option<DateTime>,
    /// 更新时间
    pub update_time: Option<DateTime>,
    /// 备注
    pub remark: Option<String>,
    /// 用户ID
    pub user_id: Option<i64>,
    ///  租户ID
    pub tenant_id: Option<i64>,
    /// 是否删除
    pub is_delete: Option<i8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum TransTextStatus {
    #[default]
    NotStart,
    Waiting,
    Processing,
    Ok,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Font {
    pub font_family: String,
    pub font_size: usize,
}
impl Default for Font {
    fn default() -> Self {
        Font {
            font_family: "Arial".to_string(),
            font_size: 14,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page_type: PaginationType,
    pub char_count: usize,
    pub delimiter: String,
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination {
            page_type: PaginationType::Auto,
            char_count: 300,
            delimiter: "\\n".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum PaginationType {
    #[default]
    Auto,
    Single,
    CharCount,
    Delimiter,
}

crud!(Workspace {});
htmlsql!(list_workspaces(rb: &dyn Executor,knowledge_base_id: i64, filter_text: Option<String>) ->Vec<Workspace> => "src/db/mapper/workspace.html");
