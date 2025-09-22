pub fn route()->Router{
     Router::new().route("/",get(get_index).post(post_index))
.route("/fun",get(test))
}