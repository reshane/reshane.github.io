import { render_post } from './mkd/pkg';
const pathName = window.location.pathname;
render_post(pathName.replace("/posts/", ""));
