
use yew::prelude::*;

#[function_component(App)]
fn app()->Html{

	let message = "Frontend";
	let backend = "api/hi";

	html!{
		<div>
			<h4>{message}</h4>
			<a href="api/hi">{backend}</a>
		</div>
	}
}


fn main() {
	yew::start_app::<App>();
}
