use diesel::{self,RunQueryDsl,QueryDsl,ExpressionMethods};
use diesel::sql_types::Integer;
use diesel::dsl::sql_query;
use actix_web::{actix::Handler, error,Error};
use chrono::Utc;
use model::response::{CategorysMsgs, Msgs, ThemePageListMsgs};
use router::ConnDsl;
use model::theme::{Theme, Save, ThemeListResult};
use model::user::User;
use utils::{time, state::PAGE_SIZE,order_vec};
use model::category::{Category, Categorys, NewCategory, CategoryNew, CategoryThemePageList};


impl Handler<CategoryNew> for ConnDsl {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, category_new: CategoryNew, _: &mut Self::Context) -> Self::Result {
        use utils::schema::categorys::dsl::*;
        let new_category = NewCategory {
            user_id: category_new.user_id,
            category_name: &category_new.category_name,
            category_name_cn: &category_new.category_name_cn,
            created_at: Utc::now().naive_utc(),
        };
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        diesel::insert_into(categorys).values(&new_category).execute(conn).map_err(error::ErrorInternalServerError)?;
        Ok(Msgs { 
                status: 200,
                message : "Article Publish Successful.".to_string(),
        })        
    }
}

impl Handler<Categorys> for ConnDsl {
    type Result = Result<CategorysMsgs, Error>;

    fn handle(&mut self, categorys: Categorys, _: &mut Self::Context) -> Self::Result {
        use utils::schema::categorys::dsl::*;
        let mut categorys_list: Vec<String> = vec![];
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let category_list = categorys.load::<Category>(conn).map_err(error::ErrorInternalServerError)?;
        Ok(CategorysMsgs { 
                status: 200,
                message : "TypeNamesMsgs.".to_string(),
                categorys: category_list,
        })        
    }
}

impl Handler<CategoryThemePageList> for ConnDsl {
    type Result = Result<ThemePageListMsgs, Error>;

    fn handle(&mut self, category_theme_page_list: CategoryThemePageList, _: &mut Self::Context) -> Self::Result {
        use utils::schema::themes::dsl::*;
        use utils::schema::users;
        use utils::schema::categorys;
        use utils::schema::saves;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        if category_theme_page_list.category_name == "care" {
            let mut themes_result = themes.filter(comment_count.eq(0)).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?;
            let theme_category_count = themes_result.len() as i32;
            let theme_page_count = (theme_category_count + PAGE_SIZE - 1) / PAGE_SIZE ;
            let mut themes_page_result = sql_query("SELECT * FROM themes WHERE themes.comment_count = 0 ORDER BY themes.id DESC limit $1 OFFSET $2")
                .bind::<Integer, _>(PAGE_SIZE)
                .bind::<Integer, _>((category_theme_page_list.page_id - 1) * PAGE_SIZE)
                .load::<Theme>(conn).map_err(error::ErrorInternalServerError)?;
            let mut category_themes_list: Vec<ThemeListResult> = vec![];
            for theme_one in themes_page_result {
                let mut category_themes_list_one = ThemeListResult::new();
                category_themes_list_one.id = theme_one.id;
                category_themes_list_one.user_id = theme_one.user_id;
                category_themes_list_one.category_id = theme_one.category_id;
                category_themes_list_one.theme_status = theme_one.theme_status;
                category_themes_list_one.title = theme_one.title;
                category_themes_list_one.content = theme_one.content;
                category_themes_list_one.view_count = theme_one.view_count;
                category_themes_list_one.comment_count = theme_one.comment_count;
                category_themes_list_one.created_at = theme_one.created_at;
                category_themes_list_one.rtime = time( Utc::now().naive_utc(), theme_one.created_at);
                let category_result =  categorys::table.filter(categorys::id.eq(theme_one.category_id)).load::<Category>(conn).map_err(error::ErrorInternalServerError)?.pop();
                match category_result {
                    Some(category_one) => { 
                        category_themes_list_one.category_name = category_one.category_name;
                        category_themes_list_one.category_name_cn = category_one.category_name_cn;
                    },
                    None => { println!("No category result");},
                }
                let theme_user =  users::table.filter(users::id.eq(theme_one.user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
                match theme_user {
                    Some(user) => {
                        category_themes_list_one.username = user.username;
                        category_themes_list.push(category_themes_list_one);
                    },            
                    None => { println!("No user result"); },
                }
            }
            let category_list =  categorys::table.load::<Category>(conn).map_err(error::ErrorInternalServerError)?;

            Ok(ThemePageListMsgs { 
                status: 200,
                message : "theme_list result.".to_string(),
                theme_list: category_themes_list,
                theme_page_count: theme_page_count,
                categorys: category_list,
            }) 
        }else if category_theme_page_list.category_name == "best" {
            let saves = saves::table.order(saves::id).load::<Save>(conn).map_err(error::ErrorInternalServerError)?;
            let mut theme_ids: Vec<i32> = vec![];   //总thme_ids
            for save in saves {
                    theme_ids.push(save.theme_id)
            }
            let mut theme_ids_page_one = theme_ids.clone();
            let mut theme_ids2 = theme_ids.clone();
            let theme_ids_result:Vec<_> = order_vec(theme_ids);
            let theme_category_count = theme_ids_result.len() as i32; // 不重复theme数量
            let theme_page_count = (theme_category_count + PAGE_SIZE  - 1) / PAGE_SIZE + 1; //页数

            if category_theme_page_list.page_id == 1 {
                theme_ids_page_one.sort_by(|a,b| b.cmp(a));
                theme_ids_page_one.dedup();
                if theme_ids_page_one.len() >= 33 {
                        let mut page_one_id_result:Vec<i32> = vec![];
                        let mut page_one_ids_33:Vec<i32> = vec![];
                        for (index,item) in theme_ids_page_one.iter().enumerate().filter(|&(idx, _)| idx < 33) {
                            page_one_ids_33.push(*item);
                        }
                        for page_one_id in page_one_ids_33.iter() {
                            for index in theme_ids2.iter() {
                                    if *page_one_id == *index{
                                        page_one_id_result.push(*index)
                                    }
                            }
                        }
                        let theme_ids_one_result:Vec<i32> = order_vec(page_one_id_result);
                        let mut category_themes_list: Vec<ThemeListResult> = vec![];
                        for index in theme_ids_one_result.iter() {
                            let theme_one_result = themes.filter(id.eq(*index)).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?.pop();
                            match theme_one_result {
                                    Some(theme_one) => {
                                        let mut category_themes_list_one = ThemeListResult::new();
                                        category_themes_list_one.id = theme_one.id;
                                        category_themes_list_one.user_id = theme_one.user_id;
                                        category_themes_list_one.category_id = theme_one.category_id;
                                        category_themes_list_one.theme_status = theme_one.theme_status;
                                        category_themes_list_one.title = theme_one.title;
                                        category_themes_list_one.content = theme_one.content;
                                        category_themes_list_one.view_count = theme_one.view_count;
                                        category_themes_list_one.comment_count = theme_one.comment_count;
                                        category_themes_list_one.created_at = theme_one.created_at;
                                        category_themes_list_one.rtime = time( Utc::now().naive_utc(), theme_one.created_at);
                                        let category_result =  categorys::table.filter(categorys::id.eq(theme_one.category_id)).load::<Category>(conn).map_err(error::ErrorInternalServerError)?.pop();
                                        match category_result {
                                            Some(category_one) => { 
                                                category_themes_list_one.category_name = category_one.category_name;
                                                category_themes_list_one.category_name_cn = category_one.category_name_cn;
                                            },
                                            None => { println!("No category result");},
                                        }
                                        let theme_user =  users::table.filter(users::id.eq(theme_one.user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
                                        match theme_user {
                                            Some(user) => {
                                                category_themes_list_one.username = user.username;
                                                category_themes_list.push(category_themes_list_one);
                                            },            
                                            None => { println!("No user result"); },
                                        }
                                    },            
                                    None => { println!("No best_theme result"); },
                            }
                        }
                        let category_list =  categorys::table.load::<Category>(conn).map_err(error::ErrorInternalServerError)?;
                        Ok(ThemePageListMsgs { 
                                status: 200,
                                message : "theme_list result.".to_string(),
                                theme_list: category_themes_list,
                                theme_page_count: theme_page_count,
                                categorys: category_list,
                        })   
                }else{
                        let mut page_one_id_result:Vec<i32> = vec![];
                        for page_one_id in theme_ids_page_one.iter() {
                            for index in theme_ids2.iter() {
                                    if *page_one_id == *index{
                                        page_one_id_result.push(*index)
                                    }
                            }
                        }
                        let theme_ids_one_result:Vec<i32> = order_vec(page_one_id_result);
                        let mut category_themes_list: Vec<ThemeListResult> = vec![];
                        for index in theme_ids_one_result.iter() {
                            let theme_one_result = themes.filter(id.eq(*index)).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?.pop();
                            match theme_one_result {
                                    Some(theme_one) => {
                                        let mut category_themes_list_one = ThemeListResult::new();
                                        category_themes_list_one.id = theme_one.id;
                                        category_themes_list_one.user_id = theme_one.user_id;
                                        category_themes_list_one.category_id = theme_one.category_id;
                                        category_themes_list_one.theme_status = theme_one.theme_status;
                                        category_themes_list_one.title = theme_one.title;
                                        category_themes_list_one.content = theme_one.content;
                                        category_themes_list_one.view_count = theme_one.view_count;
                                        category_themes_list_one.comment_count = theme_one.comment_count;
                                        category_themes_list_one.created_at = theme_one.created_at;
                                        category_themes_list_one.rtime = time( Utc::now().naive_utc(), theme_one.created_at);
                                        let category_result =  categorys::table.filter(categorys::id.eq(theme_one.category_id)).load::<Category>(conn).map_err(error::ErrorInternalServerError)?.pop();
                                        match category_result {
                                            Some(category_one) => { 
                                                category_themes_list_one.category_name = category_one.category_name;
                                                category_themes_list_one.category_name_cn = category_one.category_name_cn;
                                            },
                                            None => { println!("No category result");},
                                        }
                                        let theme_user =  users::table.filter(users::id.eq(theme_one.user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
                                        match theme_user {
                                            Some(user) => {
                                                category_themes_list_one.username = user.username;
                                                category_themes_list.push(category_themes_list_one);
                                            },            
                                            None => { println!("No user result"); },
                                        }
                                    },            
                                    None => { println!("No best_theme result"); },
                            }
                        }
                        let category_list =  categorys::table.load::<Category>(conn).map_err(error::ErrorInternalServerError)?;
                        Ok(ThemePageListMsgs { 
                                status: 200,
                                message : "theme_list result.".to_string(),
                                theme_list: category_themes_list,
                                theme_page_count: theme_page_count,
                                categorys: category_list,
                        })

                }
            }else{
                let mut category_themes_list: Vec<ThemeListResult> = vec![];
                
                if category_theme_page_list.page_id != theme_page_count {
                    let base = ((category_theme_page_list.page_id - 2) * PAGE_SIZE) as usize;
                    for index in base..(base + (PAGE_SIZE as usize)){
                        let theme_one_result = themes.filter(id.eq(theme_ids_result[index])).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?.pop();
                        match theme_one_result {
                                Some(theme_one) => {
                                    let mut category_themes_list_one = ThemeListResult::new();
                                    category_themes_list_one.id = theme_one.id;
                                    category_themes_list_one.user_id = theme_one.user_id;
                                    category_themes_list_one.category_id = theme_one.category_id;
                                    category_themes_list_one.theme_status = theme_one.theme_status;
                                    category_themes_list_one.title = theme_one.title;
                                    category_themes_list_one.content = theme_one.content;
                                    category_themes_list_one.view_count = theme_one.view_count;
                                    category_themes_list_one.comment_count = theme_one.comment_count;
                                    category_themes_list_one.created_at = theme_one.created_at;
                                    category_themes_list_one.rtime = time( Utc::now().naive_utc(), theme_one.created_at);
                                    let category_result =  categorys::table.filter(categorys::id.eq(theme_one.category_id)).load::<Category>(conn).map_err(error::ErrorInternalServerError)?.pop();
                                    match category_result {
                                        Some(category_one) => { 
                                            category_themes_list_one.category_name = category_one.category_name;
                                            category_themes_list_one.category_name_cn = category_one.category_name_cn;
                                        },
                                        None => { println!("No category result");},
                                    }
                                    let theme_user =  users::table.filter(users::id.eq(theme_one.user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
                                    match theme_user {
                                        Some(user) => {
                                            category_themes_list_one.username = user.username;
                                            category_themes_list.push(category_themes_list_one);
                                        },            
                                        None => { println!("No user result"); },
                                    }
                                },            
                                None => { println!("No best_theme result"); },
                        }
                    }
                }else{
                    let tail_num =  theme_ids_result.len() % 33;
                    let base = ((theme_page_count -2) * PAGE_SIZE) as usize;
                    if tail_num == 0{
                        for index in base..(base + (PAGE_SIZE as usize)){
                            let theme_one_result = themes.filter(id.eq(theme_ids_result[index])).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?.pop();
                            match theme_one_result {
                                    Some(theme_one) => {
                                        let mut category_themes_list_one = ThemeListResult::new();
                                        category_themes_list_one.id = theme_one.id;
                                        category_themes_list_one.user_id = theme_one.user_id;
                                        category_themes_list_one.category_id = theme_one.category_id;
                                        category_themes_list_one.theme_status = theme_one.theme_status;
                                        category_themes_list_one.title = theme_one.title;
                                        category_themes_list_one.content = theme_one.content;
                                        category_themes_list_one.view_count = theme_one.view_count;
                                        category_themes_list_one.comment_count = theme_one.comment_count;
                                        category_themes_list_one.created_at = theme_one.created_at;
                                        category_themes_list_one.rtime = time( Utc::now().naive_utc(), theme_one.created_at);
                                        let category_result =  categorys::table.filter(categorys::id.eq(theme_one.category_id)).load::<Category>(conn).map_err(error::ErrorInternalServerError)?.pop();
                                        match category_result {
                                            Some(category_one) => { 
                                                category_themes_list_one.category_name = category_one.category_name;
                                                category_themes_list_one.category_name_cn = category_one.category_name_cn;
                                            },
                                            None => { println!("No category result");},
                                        }
                                        let theme_user =  users::table.filter(users::id.eq(theme_one.user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
                                        match theme_user {
                                            Some(user) => {
                                                category_themes_list_one.username = user.username;
                                                category_themes_list.push(category_themes_list_one);
                                            },            
                                            None => { println!("No user result"); },
                                        }
                                    },            
                                    None => { println!("No best_theme result"); },
                            }
                        }
                    }else{
                        for index in base..(base + (tail_num as usize)){
                            let theme_one_result = themes.filter(id.eq(theme_ids_result[index])).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?.pop();
                            match theme_one_result {
                                    Some(theme_one) => {
                                        let mut category_themes_list_one = ThemeListResult::new();
                                        category_themes_list_one.id = theme_one.id;
                                        category_themes_list_one.user_id = theme_one.user_id;
                                        category_themes_list_one.category_id = theme_one.category_id;
                                        category_themes_list_one.theme_status = theme_one.theme_status;
                                        category_themes_list_one.title = theme_one.title;
                                        category_themes_list_one.content = theme_one.content;
                                        category_themes_list_one.view_count = theme_one.view_count;
                                        category_themes_list_one.comment_count = theme_one.comment_count;
                                        category_themes_list_one.created_at = theme_one.created_at;
                                        category_themes_list_one.rtime = time( Utc::now().naive_utc(), theme_one.created_at);
                                        let category_result =  categorys::table.filter(categorys::id.eq(theme_one.category_id)).load::<Category>(conn).map_err(error::ErrorInternalServerError)?.pop();
                                        match category_result {
                                            Some(category_one) => { 
                                                category_themes_list_one.category_name = category_one.category_name;
                                                category_themes_list_one.category_name_cn = category_one.category_name_cn;
                                            },
                                            None => { println!("No category result");},
                                        }
                                        let theme_user =  users::table.filter(users::id.eq(theme_one.user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
                                        match theme_user {
                                            Some(user) => {
                                                category_themes_list_one.username = user.username;
                                                category_themes_list.push(category_themes_list_one);
                                            },            
                                            None => { println!("No user result"); },
                                        }
                                    },            
                                    None => { println!("No best_theme result"); },
                            }
                        }
                    }
                    
                }

                let category_list =  categorys::table.load::<Category>(conn).map_err(error::ErrorInternalServerError)?;
                Ok(ThemePageListMsgs { 
                        status: 200,
                        message : "theme_list result.".to_string(),
                        theme_list: category_themes_list,
                        theme_page_count: theme_page_count,
                        categorys: category_list,
                })  
            }   
        }else {
            let theme_category_one = categorys::table.filter(&categorys::category_name.eq(&category_theme_page_list.category_name)).load::<Category>(conn).map_err(error::ErrorInternalServerError)?.pop();
            let the_category_id = match theme_category_one {
                Some(theme_category_id) => theme_category_id.id,
                None => 0,
            };
            let mut themes_result = themes.filter(category_id.eq(the_category_id)).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?;
            let theme_category_count = themes_result.len() as i32;
            let theme_page_count = (theme_category_count + PAGE_SIZE - 1) / PAGE_SIZE ;
            let mut themes_page_result = sql_query("SELECT * FROM themes WHERE themes.category_id = $1 ORDER BY themes.id DESC limit $2 OFFSET $3")
                .bind::<Integer, _>(the_category_id)
                .bind::<Integer, _>(PAGE_SIZE)
                .bind::<Integer, _>((category_theme_page_list.page_id - 1) * PAGE_SIZE)
                .load::<Theme>(conn).map_err(error::ErrorInternalServerError)?;
            let mut category_themes_list: Vec<ThemeListResult> = vec![];
            for theme_one in themes_page_result {
                let mut category_themes_list_one = ThemeListResult::new();
                category_themes_list_one.id = theme_one.id;
                category_themes_list_one.user_id = theme_one.user_id;
                category_themes_list_one.category_id = theme_one.category_id;
                category_themes_list_one.theme_status = theme_one.theme_status;
                category_themes_list_one.title = theme_one.title;
                category_themes_list_one.content = theme_one.content;
                category_themes_list_one.view_count = theme_one.view_count;
                category_themes_list_one.comment_count = theme_one.comment_count;
                category_themes_list_one.created_at = theme_one.created_at;
                category_themes_list_one.rtime = time( Utc::now().naive_utc(), theme_one.created_at);
                let category_result =  categorys::table.filter(categorys::id.eq(theme_one.category_id)).load::<Category>(conn).map_err(error::ErrorInternalServerError)?.pop();
                match category_result {
                    Some(category_one) => { 
                        category_themes_list_one.category_name = category_one.category_name;
                        category_themes_list_one.category_name_cn = category_one.category_name_cn;
                    },
                    None => { println!("No category result");},
                }
                let theme_user =  users::table.filter(users::id.eq(theme_one.user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
                match theme_user {
                    Some(user) => {
                        category_themes_list_one.username = user.username;
                        category_themes_list.push(category_themes_list_one);
                    },            
                    None => { println!("No user result"); },
                }
            }
            let category_list =  categorys::table.load::<Category>(conn).map_err(error::ErrorInternalServerError)?;
            Ok(ThemePageListMsgs { 
                status: 200,
                message : "theme_list result.".to_string(),
                theme_list: category_themes_list,
                theme_page_count: theme_page_count,
                categorys: category_list,
            }) 
        }

    }
}
