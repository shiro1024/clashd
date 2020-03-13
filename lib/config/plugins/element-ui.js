import Vue from "vue";
import { Form, FormItem, Input, Col, Row, Button } from "element-ui";
import lang from "element-ui/lib/locale/lang/zh-CN";
import locale from "element-ui/lib/locale";

// 设置语言
locale.use(lang);

export default () => {
  Vue.use(Form);
  Vue.use(FormItem);
  Vue.use(Input);
  Vue.use(Col);
  Vue.use(Row);
  Vue.use(Button);
};
