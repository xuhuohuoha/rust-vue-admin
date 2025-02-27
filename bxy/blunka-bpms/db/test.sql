/*
Navicat MySQL Data Transfer

Source Server         : localhost_3306
Source Server Version : 50719
Source Host           : localhost:4455
Source Database       : blunka_test

Target Server Type    : MYSQL
Target Server Version : 50719
File Encoding         : 65001

Date: 2024-12-06 13:13:40
*/

SET FOREIGN_KEY_CHECKS=0;

-- ----------------------------
-- Table structure for `bxy_adtion`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_adtion`;
CREATE TABLE `bxy_adtion` (
  `id` varchar(255) NOT NULL,
  `guid` varchar(255) NOT NULL,
  `create_by` varchar(255) NOT NULL,
  `delete_by` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `deleted_at` datetime DEFAULT NULL,
  `mcode` varchar(255) NOT NULL,
  `fname` varchar(255) NOT NULL,
  `ext` varchar(255) NOT NULL,
  `fsize` varchar(255) NOT NULL,
  `ucode` varchar(255) NOT NULL,
  `url` varchar(255) NOT NULL,
  `ext1` varchar(255) DEFAULT NULL,
  `ext2` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_adtion
-- ----------------------------

-- ----------------------------
-- Table structure for `bxy_adtion_ex`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_adtion_ex`;
CREATE TABLE `bxy_adtion_ex` (
  `id` varchar(255) NOT NULL,
  `guid` varchar(255) NOT NULL,
  `pguid` varchar(255) NOT NULL,
  `create_by` varchar(255) NOT NULL,
  `update_by` varchar(255) DEFAULT NULL,
  `delete_by` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime DEFAULT NULL,
  `deleted_at` datetime DEFAULT NULL,
  `version` int(10) unsigned NOT NULL,
  `ord` int(10) unsigned NOT NULL,
  `status` varchar(255) NOT NULL,
  `remark` varchar(255) DEFAULT NULL,
  `mcode` varchar(255) NOT NULL,
  `exname` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_adtion_ex
-- ----------------------------
INSERT INTO `bxy_adtion_ex` VALUES ('03CYHLV2AS2Q4DCCIQPY0OZ1P', '03CYHLV2AS2Q4DCCIQT8WB3CN', '03CYHLV2AS2Q4DCCIQT8WB3CN', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-05 10:19:47', null, null, '1', '0', '1', '', '03cv1s3mjc4w2ef857yjrqhng', '头像');

-- ----------------------------
-- Table structure for `bxy_app`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_app`;
CREATE TABLE `bxy_app` (
  `id` varchar(255) NOT NULL,
  `guid` varchar(50) DEFAULT NULL,
  `create_by` varchar(255) NOT NULL,
  `update_by` varchar(255) DEFAULT NULL,
  `delete_by` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime DEFAULT NULL,
  `deleted_at` datetime DEFAULT NULL,
  `version` int(10) unsigned NOT NULL,
  `ord` int(10) unsigned NOT NULL,
  `status` varchar(255) NOT NULL,
  `remark` varchar(255) NOT NULL,
  `app_code` varchar(255) NOT NULL,
  `app_key` varchar(255) NOT NULL,
  `app_name` varchar(255) NOT NULL,
  `app_att` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_app
-- ----------------------------
INSERT INTO `bxy_app` VALUES ('03ctywcl3sjqt4l75nzfh495n', '03ctywcl3sjqt4l75nzfh495n', '1', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-14 14:34:23', '2024-11-28 10:54:03', null, '4', '100', '1', '', '03cv1mn38vhrkeli8jabfeej2', '03cv1mogvphd42f4uyqamgfpc', '开发平台', '');

-- ----------------------------
-- Table structure for `bxy_app_auth`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_app_auth`;
CREATE TABLE `bxy_app_auth` (
  `id` varchar(255) NOT NULL,
  `app_code` varchar(255) NOT NULL,
  `temp_code` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_app_auth
-- ----------------------------
INSERT INTO `bxy_app_auth` VALUES ('03cv93d0w1rv0pltubnu51jyo', '03ctywcl3sjqt4l75nzfh495n', '03cv93d0w1rv0pltubppzw8bh');
INSERT INTO `bxy_app_auth` VALUES ('03cv9i6d14ofyq2jymyha43nn', '03ctywcl3sjqt4l75nzfh495n', '03cv9i6d14ofyq2jymzei7owh');
INSERT INTO `bxy_app_auth` VALUES ('03cvhx5gy4qj7h7jwt8s14vtb', '03ctywcl3sjqt4l75nzfh495n', '03cvhx5gy4qj7h7jwtatrgbz4');
INSERT INTO `bxy_app_auth` VALUES ('03cvhxfnmzs5bkuepqourgzwc', '03ctywcl3sjqt4l75nzfh495n', '03cvhxfnmzs5bkuepqrn1tf4q');
INSERT INTO `bxy_app_auth` VALUES ('03cvhzub3n5ihrndopmnezzgb', '03ctywcl3sjqt4l75nzfh495n', '03cvhzub3n5ihrndopq4fq1dl');
INSERT INTO `bxy_app_auth` VALUES ('03cvhzuz0k3830wgy97nc9jeu', '03ctywcl3sjqt4l75nzfh495n', '03cvhzuz0k3830wgy9atw177i');
INSERT INTO `bxy_app_auth` VALUES ('03cvhzvj0frm4z4pv46d02fcl', '03ctywcl3sjqt4l75nzfh495n', '03cvhzvj0frm4z4pv46sng1n6');
INSERT INTO `bxy_app_auth` VALUES ('03cvi3cu8cej5ng8wrdqw3ms6', '03ctywcl3sjqt4l75nzfh495n', '03cvi3cu8cej5ng8wrg5r0hrs');
INSERT INTO `bxy_app_auth` VALUES ('03cvi3dd7bn2pqxabtxpqxxwe', '03ctywcl3sjqt4l75nzfh495n', '03cvi3dd7bn2pqxabu0tt0gzk');
INSERT INTO `bxy_app_auth` VALUES ('03cvi50lmwz1ilgomyslywd43', '03ctywcl3sjqt4l75nzfh495n', '03cvi50lmwz1ilgomyu5f286m');
INSERT INTO `bxy_app_auth` VALUES ('03cvi5rk0ea1zgcbaq3f8rfd2', '03ctywcl3sjqt4l75nzfh495n', '03cvi5rk0ea1zgcbaq6lavgr0');
INSERT INTO `bxy_app_auth` VALUES ('03cvi5terh0vobvuiriyyck62', '03ctywcl3sjqt4l75nzfh495n', '03cvi5terh0vobvuirlh19vob');
INSERT INTO `bxy_app_auth` VALUES ('03cvi5u0ofovfv8nrtpyvggzy', '03ctywcl3sjqt4l75nzfh495n', '03cvi5u0ofovfv8nrtst05elj');
INSERT INTO `bxy_app_auth` VALUES ('03cvi5ug6purz4kkpcbkzdw7d', '03ctywcl3sjqt4l75nzfh495n', '03cvi5ug6purz4kkpcdhr2jqz');
INSERT INTO `bxy_app_auth` VALUES ('03cvi64xlxi2rwli6o7vbso23', '03ctywcl3sjqt4l75nzfh495n', '03cvi64xlxi2rwli6obeqjgir');
INSERT INTO `bxy_app_auth` VALUES ('03cvi64zhtvy4ublidwll5tdq', '03ctywcl3sjqt4l75nzfh495n', '03cvi64zhtvy4ublidygqogtl');
INSERT INTO `bxy_app_auth` VALUES ('03cvi9hhenua6yau6h5r9vjrz', '03ctywcl3sjqt4l75nzfh495n', '03cvi9hhenua6yau6h6ndhkmx');
INSERT INTO `bxy_app_auth` VALUES ('03cvib0ctfvyxkmr61i01aags', '03ctywcl3sjqt4l75nzfh495n', '03cvib0ctfvyxkmr61j8q310t');
INSERT INTO `bxy_app_auth` VALUES ('03cvib0wruxig6c037m2wcggb', '03ctywcl3sjqt4l75nzfh495n', '03cvib0wruxig6c037oma9b27');
INSERT INTO `bxy_app_auth` VALUES ('03cvib1nl3lmaw35q2i8ckukp', '03ctywcl3sjqt4l75nzfh495n', '03cvib1nl3lmaw35q2j4b5wws');
INSERT INTO `bxy_app_auth` VALUES ('03cvnm3n8jgb3vx0cnkl86rgx', '03ctywcl3sjqt4l75nzfh495n', '03cvnm3n8jgb3vx0cnnui1xi9');
INSERT INTO `bxy_app_auth` VALUES ('03cvnm437q2r7k887q5a2ygq6', '03ctywcl3sjqt4l75nzfh495n', '03cvnm437q2r7k887q6oyyxev');
INSERT INTO `bxy_app_auth` VALUES ('03cvnm4inmtnj6q4tjrm6rdzu', '03ctywcl3sjqt4l75nzfh495n', '03cvnm4inmtnj6q4tjsbwah0e');
INSERT INTO `bxy_app_auth` VALUES ('03cvnm4v8uwhrhnlo8rfz7p8k', '03ctywcl3sjqt4l75nzfh495n', '03cvnm4v8uwhrhnlo8sq02p9e');
INSERT INTO `bxy_app_auth` VALUES ('03cvp4iwm2mvmdk5q9x9kc90s', '03ctywcl3sjqt4l75nzfh495n', '03cvp4iwm2mvmdk5qa0nh7boe');
INSERT INTO `bxy_app_auth` VALUES ('03cvp4nl9v0oqvuxxhtp0dnhd', '03ctywcl3sjqt4l75nzfh495n', '03cvp4nl9v0oqvuxxhvyaprho');
INSERT INTO `bxy_app_auth` VALUES ('03cvpbanb9vjxy3rkvrwdyqaj', '03ctywcl3sjqt4l75nzfh495n', '03cvpbanb9vjxy3rkvtoenmxg');
INSERT INTO `bxy_app_auth` VALUES ('03cvpbbo7n50svxk3o5ydqcbl', '03ctywcl3sjqt4l75nzfh495n', '03cvpbbo7n50svxk3o7w59z0w');
INSERT INTO `bxy_app_auth` VALUES ('03cvpbc25m73ptnmtzeylwaem', '03ctywcl3sjqt4l75nzfh495n', '03cvpbc25m73ptnmtzgeaw31d');
INSERT INTO `bxy_app_auth` VALUES ('03cvpbcun52rkzm7v79a8r893', '03ctywcl3sjqt4l75nzfh495n', '03cvpbcun52rkzm7v7c5ol4hg');
INSERT INTO `bxy_app_auth` VALUES ('03cvpbkawuquynott65mznjo4', '03ctywcl3sjqt4l75nzfh495n', '03cvpbkawuquynott66sap44e');
INSERT INTO `bxy_app_auth` VALUES ('03cvpbn9vgjx6tkr8v0hqkott', '03ctywcl3sjqt4l75nzfh495n', '03cvpbn9vgjx6tkr8v2gxux89');
INSERT INTO `bxy_app_auth` VALUES ('03cvpbo6a11r6rrj3z526g4go', '03ctywcl3sjqt4l75nzfh495n', '03cvpbo6a11r6rrj3z6embfxn');
INSERT INTO `bxy_app_auth` VALUES ('03cvpbparc1ttnn348jbg7t3s', '03ctywcl3sjqt4l75nzfh495n', '03cvpbparc1ttnn348lvy351g');
INSERT INTO `bxy_app_auth` VALUES ('03cwb36s0rsua8b0vfl6pn4wf', '03ctywcl3sjqt4l75nzfh495n', '03cwb36s0rsua8b0vfooa5n05');
INSERT INTO `bxy_app_auth` VALUES ('03cwb37b367rvflbfuha31qer', '03ctywcl3sjqt4l75nzfh495n', '03cwb37b367rvflbfui5e2ci4');
INSERT INTO `bxy_app_auth` VALUES ('03cwb37poc31m3mveqs1h8so6', '03ctywcl3sjqt4l75nzfh495n', '03cwb37poc31m3mveqtgkcown');
INSERT INTO `bxy_app_auth` VALUES ('03cwb382t1rpxb3tv8nla1g2b', '03ctywcl3sjqt4l75nzfh495n', '03cwb382t1rpxb3tv8pnnh3c1');
INSERT INTO `bxy_app_auth` VALUES ('03cwb38wausfqf7gyeh01bmc7', '03ctywcl3sjqt4l75nzfh495n', '03cwb38wausfqf7gyeixwcof4');
INSERT INTO `bxy_app_auth` VALUES ('03cwb3wtsjct1mnjboh013sme', '03ctywcl3sjqt4l75nzfh495n', '03cwb3wtsjct1mnjbokfsdf49');
INSERT INTO `bxy_app_auth` VALUES ('03cwb3xaumlolbabgudlwmgv0', '03ctywcl3sjqt4l75nzfh495n', '03cwb3xaumlolbabgufj3ofmx');
INSERT INTO `bxy_app_auth` VALUES ('03cwb3xp6vl4a8y9py5jbv2y2', '03ctywcl3sjqt4l75nzfh495n', '03cwb3xp6vl4a8y9py6zor3mg');
INSERT INTO `bxy_app_auth` VALUES ('03cwb3y2q4pxvmniprzfh24a9', '03ctywcl3sjqt4l75nzfh495n', '03cwb3y2q4pxvmnips0so9gvx');
INSERT INTO `bxy_app_auth` VALUES ('03cwb3y43yd339zeqmp5amo46', '03ctywcl3sjqt4l75nzfh495n', '03cwb3y43yd339zeqmsbtryrf');
INSERT INTO `bxy_app_auth` VALUES ('03cwdec2x4uounauzjwebatpe', '03ctywcl3sjqt4l75nzfh495n', '03cwdec2x4uounauzjzv2200f');
INSERT INTO `bxy_app_auth` VALUES ('03cws5tcjwlm2s004tm226x5v', '03ctywcl3sjqt4l75nzfh495n', '03cws5tcjwlm2s004tnpfssaf');
INSERT INTO `bxy_app_auth` VALUES ('03cy9w38j6kcsnp4zee8eyhbj', '03ctywcl3sjqt4l75nzfh495n', '03cy9w38j6kcsnp4zegc1o695');
INSERT INTO `bxy_app_auth` VALUES ('03cy9w3scvtj36o47myp9sv2o', '03ctywcl3sjqt4l75nzfh495n', '03cy9w3scvtj36o47n0t0c9tt');
INSERT INTO `bxy_app_auth` VALUES ('03cy9w4b1kizcyy2vzq9a9ovt', '03ctywcl3sjqt4l75nzfh495n', '03cy9w4b1kizcyy2vztevwpgs');
INSERT INTO `bxy_app_auth` VALUES ('03cya02ijysgd75zdsedwg8mt', '03ctywcl3sjqt4l75nzfh495n', '03cya02ijysgd75zdsh4dsqpo');
INSERT INTO `bxy_app_auth` VALUES ('03cyq9lnpe4qytiarn9tmf9st', '03ctywcl3sjqt4l75nzfh495n', '03cyq9lnpe4qytiarncumgd8c');
INSERT INTO `bxy_app_auth` VALUES ('03cyqb76js87nyxzlac1k9je4', '03ctywcl3sjqt4l75nzfh495n', '03cyqb76js87nyxzlaend8wam');

-- ----------------------------
-- Table structure for `bxy_col_auth`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_col_auth`;
CREATE TABLE `bxy_col_auth` (
  `id` varchar(255) NOT NULL,
  `create_by` varchar(255) NOT NULL,
  `update_by` varchar(255) DEFAULT NULL,
  `delete_by` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime DEFAULT NULL,
  `deleted_at` datetime DEFAULT NULL,
  `version` int(10) unsigned NOT NULL,
  `ord` int(10) unsigned NOT NULL,
  `status` varchar(255) NOT NULL,
  `remark` varchar(255) NOT NULL,
  `field` varchar(255) NOT NULL,
  `regx` varchar(255) NOT NULL,
  `mcode` varchar(255) NOT NULL,
  `atype` tinyint(3) unsigned NOT NULL,
  `amethod` smallint(5) unsigned NOT NULL,
  `u_id` varchar(255) NOT NULL,
  `r_id` varchar(255) NOT NULL,
  `o_id` varchar(255) NOT NULL,
  `p_id` varchar(255) NOT NULL,
  `d_id` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_col_auth
-- ----------------------------
INSERT INTO `bxy_col_auth` VALUES ('03CYHEY5TMLVUULD1YGB8Y0NJ', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-05 09:44:24', '2024-12-05 14:29:15', null, '2', '0', '1', '', 'api', 'regx', '03cv1rwvjw9hqcmsxy5axxzyn', '0', '0', '', '', '', '', '');

-- ----------------------------
-- Table structure for `bxy_datasource`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_datasource`;
CREATE TABLE `bxy_datasource` (
  `id` varchar(255) NOT NULL,
  `create_by` varchar(255) NOT NULL,
  `update_by` varchar(255) DEFAULT NULL,
  `delete_by` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime DEFAULT NULL,
  `deleted_at` datetime DEFAULT NULL,
  `version` int(10) unsigned NOT NULL,
  `ord` int(10) unsigned NOT NULL,
  `status` varchar(255) NOT NULL,
  `remark` varchar(255) NOT NULL,
  `db` varchar(255) NOT NULL,
  `url` varchar(255) NOT NULL,
  `maxc` int(10) unsigned NOT NULL,
  `minc` int(10) unsigned NOT NULL,
  `conn_timeout` bigint(20) unsigned NOT NULL,
  `idle_timeout` bigint(20) unsigned NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_datasource
-- ----------------------------

-- ----------------------------
-- Table structure for `bxy_dict`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_dict`;
CREATE TABLE `bxy_dict` (
  `id` varchar(255) NOT NULL,
  `guid` varchar(255) NOT NULL,
  `pguid` varchar(255) NOT NULL,
  `create_by` varchar(255) NOT NULL,
  `update_by` varchar(255) DEFAULT NULL,
  `delete_by` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime DEFAULT NULL,
  `deleted_at` datetime DEFAULT NULL,
  `version` int(10) unsigned NOT NULL,
  `ord` int(10) unsigned NOT NULL,
  `status` varchar(255) NOT NULL,
  `remark` varchar(255) NOT NULL,
  `dname` varchar(255) NOT NULL,
  `att` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_dict
-- ----------------------------
INSERT INTO `bxy_dict` VALUES ('03CWDF22WI9Y1ZI68IVCPM929', 'DATA_STATUS', 'DATA_STATUS', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-25 16:19:00', '2024-11-26 23:03:46', null, '17', '0', '1', '通用数据状态，包括：0停用、1正常、2逻辑删除', '数据状态', '');
INSERT INTO `bxy_dict` VALUES ('03CWDFFDU8D3NC5V4LKWMA9J8', 'MENU_TYPE', 'MENU_TYPE', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-25 16:20:53', '2024-11-27 14:55:45', null, '5', '0', '1', '平台预定义菜单类型，后台仅指定目录和功能两种，其他规则由前端根据实际情况指定', '菜单类型', '');
INSERT INTO `bxy_dict` VALUES ('03CWDH9ZKUUMQVIQRAL5DVEVR', 'HTTP_METHOD', 'HTTP_METHOD', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-25 16:30:21', '2024-11-25 17:48:22', null, '3', '0', '1', 'HTTP常用请求类型，包括：GET、PUT、POST、DELETE', 'HTTP请求类型', '');
INSERT INTO `bxy_dict` VALUES ('03CXX3IWDY8B7WUPKF2FNPA9S', 'AUTH_TYPE', 'AUTH_TYPE', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-02 19:20:08', '2024-12-02 19:20:46', null, '2', '0', '1', '平台预定义授权类型', '授权类型', '');
INSERT INTO `bxy_dict` VALUES ('03CY1NQENCE0PT1C3FANQPXA6', 'AUTH_METHOD', 'AUTH_METHOD', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-03 09:20:44', '2024-12-03 09:21:05', null, '2', '0', '1', '平台预定义授权方法', '授权方法', '');
INSERT INTO `bxy_dict` VALUES ('03CY1XNMETL9NW3OQCHXQIKBB', 'ROUTE_MODEL', 'ROUTE_MODEL', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-03 10:11:32', '2024-12-03 10:11:58', null, '2', '0', '1', '平台支持的路由方式，包括：内链、外链', '路由方式', '');
INSERT INTO `bxy_dict` VALUES ('03CY21EM0MTIFZZA4UQFJXLUL', 'OPEN_MODEL', 'OPEN_MODEL', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 10:30:44', null, null, '1', '0', '1', '平台预定义页面打开方式', '打开方式', '');
INSERT INTO `bxy_dict` VALUES ('03CY26LOJUTLRFH0A5JJAIRZ7', 'BUTTON_SCOPE', 'BUTTON_SCOPE', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-03 10:57:20', '2024-12-03 14:25:27', null, '2', '0', '1', '平台预置通用功能按钮所在区域', '功能区域', '');
INSERT INTO `bxy_dict` VALUES ('03CY2YKMLUJBY46URRPZFO0Q7', 'SEX', 'SEX', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-03 13:20:31', '2024-12-03 13:23:50', null, '4', '0', '1', '数据字典 ——> 性别', '性别', '');
INSERT INTO `bxy_dict` VALUES ('03CY3BFQS1RL0QNC6CXHL7KMK', 'BUTTON_TYPE', 'BUTTON_TYPE', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 14:26:22', null, null, '1', '0', '1', '功能按钮样式类型', '按钮类型', '');
INSERT INTO `bxy_dict` VALUES ('03CYP4WTJ1C72CTJZPBVIXOZ8', 'LOG_METHOD', 'LOG_METHOD', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-06 09:27:19', null, null, '1', '0', '1', '', '日志记录方式', '');

-- ----------------------------
-- Table structure for `bxy_dict_data`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_dict_data`;
CREATE TABLE `bxy_dict_data` (
  `id` varchar(255) NOT NULL,
  `guid` varchar(255) NOT NULL,
  `pguid` varchar(255) NOT NULL,
  `create_by` varchar(255) NOT NULL,
  `update_by` varchar(255) DEFAULT NULL,
  `delete_by` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime DEFAULT NULL,
  `deleted_at` datetime DEFAULT NULL,
  `version` int(10) unsigned NOT NULL,
  `ord` int(10) unsigned NOT NULL,
  `status` varchar(255) NOT NULL,
  `remark` varchar(255) NOT NULL,
  `lvl` tinyint(3) unsigned NOT NULL,
  `dname` varchar(255) NOT NULL,
  `att` varchar(255) NOT NULL,
  `ext1` varchar(255) NOT NULL,
  `ext2` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_dict_data
-- ----------------------------
INSERT INTO `bxy_dict_data` VALUES ('03CWSK4ZR2TQ89R694E1IG9UT', '0', '0', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 14:49:20', null, null, '1', '0', '1', '通用数据状态', '0', 'DATA_STATUS', '停用', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CWSKPZ25RMG112C3F94EYOT', '1', '1', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-27 14:52:19', '2024-12-03 09:29:45', null, '3', '1', '1', '通用数据状态', '0', 'DATA_STATUS', '正常', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CWSKT6JMCUI332UZJC7VYQO', '2', '2', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-27 14:52:47', '2024-12-03 09:29:51', null, '2', '2', '1', '通用数据状态', '0', 'DATA_STATUS', '逻辑删除', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CWSMF8MFNRV97N1Q3UCT97N', 'F100', 'F100', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-27 15:01:02', '2024-12-03 09:42:21', null, '3', '100', '1', '由前端定义的预置功能，用于当前行数据传递操作', '0', 'MENU_TYPE', '默认', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CWSMKPO7ZZR5GB30PL5F7TU', 'F110', 'F110', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-27 15:01:49', '2024-12-03 09:30:14', null, '3', '110', '1', '由前端定义的预置功能，用于打开新增组件并调用新增接口', '0', 'MENU_TYPE', '新增', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CWSMMCKSWOSVLHC6O6DYNO2', 'F120', 'F120', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-27 15:02:03', '2024-12-03 09:30:19', null, '3', '120', '1', '由前端定义的预置功能，用于打开编辑组件并调用编辑接口', '0', 'MENU_TYPE', '编辑', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CWSMOT3ANLY9M6NFZV2EUSG', 'F130', 'F130', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-27 15:02:24', '2024-12-03 09:30:31', null, '3', '130', '1', '由前端定义的预置功能，用于打开浏览组件', '0', 'MENU_TYPE', '浏览', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CWSMR835OAW6H9NJH18QZAT', 'F140', 'F140', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-27 15:02:44', '2024-12-03 09:30:48', null, '3', '140', '1', '由前端定义的预置功能，用于调用逻辑删除接口', '0', 'MENU_TYPE', '软删除', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CWSMTLH7R2F6Z15DAE1VMKH', 'F150', 'F150', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-27 15:03:04', '2024-12-03 09:30:56', null, '3', '150', '1', '由前端定义的预置功能，用于调用物理删除接口', '0', 'MENU_TYPE', '硬删除', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1NM77V9T55LNGTIG2VRXE', '0', '0', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-03 09:20:08', '2024-12-03 09:22:13', null, '2', '0', '1', '包含授权配置下的所有用户', '0', 'AUTH_TYPE', '包含', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1O61BYKVF4SRS5ZI3QFZA', '1', '1', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-03 09:22:58', '2024-12-03 09:23:23', null, '2', '1', '1', '排除授权配置下的所有用户', '0', 'AUTH_TYPE', '排除', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1OQD6UIVM2M4VNXB2HONW', '0', '0', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 09:25:51', null, null, '1', '0', '1', '授权给所有人', '0', 'AUTH_METHOD', '所有人', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1Q01TTZSDBK2BBC0GMSBQ', '1', '1', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 09:32:21', null, null, '1', '1', '1', '授权给指定用户', '0', 'AUTH_METHOD', '指定用户', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1Q24LEKX2847KYEA3VUE7', '2', '2', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 09:32:39', null, null, '1', '2', '1', '授权给指定角色', '0', 'AUTH_METHOD', '指定角色', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1Q4HUTW2OAA2O42SML3KC', '3', '3', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 09:32:59', null, null, '1', '3', '1', '授权给指定部门', '0', 'AUTH_METHOD', '指定部门', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1Q91LLSC1LC237P42HZH8', '4', '4', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 09:33:38', null, null, '1', '4', '1', '授权给指定岗位', '0', 'AUTH_METHOD', '指定岗位', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1QA4NRPOVCO62CIB8QNGM', '5', '5', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 09:33:47', null, null, '1', '5', '1', '授权给指定职务', '0', 'AUTH_METHOD', '指定职务', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1QXBDVCMSX15BGLUBAOQG', '24', '24', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-03 09:37:05', '2024-12-03 09:40:04', null, '2', '24', '1', '授权给指定角色、指定岗位', '0', 'AUTH_METHOD', '指定角色、岗位', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1QZMPQU1BTG2Y1X4WSD8D', '25', '25', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 09:37:24', null, null, '1', '25', '1', '授权给指定角色、指定职务', '0', 'AUTH_METHOD', '指定角色、职务', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1R1ZU53K51HC3TIXPOH1I', '32', '32', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 09:37:45', null, null, '1', '32', '1', '授权给指定部门、指定角色', '0', 'AUTH_METHOD', '指定部门、角色', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1R3VV364EVO9GXPRK5BXW', '34', '34', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 09:38:01', null, null, '1', '34', '1', '授权给指定部门、指定岗位', '0', 'AUTH_METHOD', '指定部门、岗位', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1R5HU986SVI32VPDQL4T2', '35', '35', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 09:38:15', null, null, '1', '35', '1', '授权给指定部门、指定职务', '0', 'AUTH_METHOD', '指定部门、职务', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1R8QBQF3QRMNRPX8IEITS', '45', '45', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 09:38:42', null, null, '1', '45', '1', '授权给指定岗位、指定职务', '0', 'AUTH_METHOD', '指定岗位、职务', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1RCFXDER6LY86IG0V8PO3', '324', '324', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 09:39:14', null, null, '1', '324', '1', '授权给指定部门、指定角色、指定岗位', '0', 'AUTH_METHOD', '指定部门、角色、岗位', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1RDY2PDW8WV689TEOIKLU', '325', '325', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 09:39:27', null, null, '1', '325', '1', '授权给指定部门、指定角色、指定职务', '0', 'AUTH_METHOD', '指定部门、角色、职务', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1RGIUEYX4J7139STFW8HS', '3245', '3245', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 09:39:49', null, null, '1', '3245', '1', '授权给指定部门、指定角色、指定岗位、指定职务', '0', 'AUTH_METHOD', '指定部门、角色、岗位、职务', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1XYJ9KPT9ASCVBCA4667S', '0', '0', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 10:13:05', null, null, '1', '0', '1', '平台内部路由', '0', 'ROUTE_MODEL', '内链', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1Y0CUDVT17C7KIKEDC2IZ', '1', '1', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 10:13:21', null, null, '1', '1', '1', '平台外部路由', '0', 'ROUTE_MODEL', '外链', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1ZLIZAKSH099L7SADUN6V', 'GET', 'GET', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 10:21:28', null, null, '1', '0', '1', 'HTTP请求类型', '0', 'HTTP_METHOD', 'GET请求', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1ZMGU4S7LIUESLYZV4EX8', 'PUT', 'PUT', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 10:21:37', null, null, '1', '0', '1', 'HTTP请求类型', '0', 'HTTP_METHOD', 'PUT请求', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1ZNQOOXX386986RKAF2QJ', 'POST', 'POST', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 10:21:47', null, null, '1', '0', '1', 'HTTP请求类型', '0', 'HTTP_METHOD', 'POST请求', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY1ZOUIHNARCKIGTIVZGUBL', 'DELETE', 'DELETE', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 10:21:57', null, null, '1', '0', '1', 'HTTP请求类型', '0', 'HTTP_METHOD', 'DELETE请求', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY21QB3M4U3PGCM854P6MU0', '0', '0', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 10:32:24', null, null, '1', '0', '1', '内部Tab页签打开', '0', 'OPEN_MODEL', 'Tab', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY21S7E3JF3UHULIT5VN2I2', '1', '1', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 10:32:40', null, null, '1', '0', '1', '浏览器新页签打开', '0', 'OPEN_MODEL', 'Page', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY21TYNAW2L65P7KXFKU5G2', '2', '2', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 10:32:55', null, null, '1', '0', '1', '内部模态框打开', '0', 'OPEN_MODEL', 'Dialog', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY26TEJZUEC6P8BPCPBY5FH', '0', '0', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 10:58:26', null, null, '1', '0', '1', '功能按钮显示在列表工具栏', '0', 'BUTTON_SCOPE', '列表工具栏', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY26V6ZQW3OZF23XK78SMY5', '1', '1', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 10:58:41', null, null, '1', '1', '1', '功能按钮显示在列表操作列', '0', 'BUTTON_SCOPE', '列表操作列', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY26XK7ARUG04E9W9Q0L0EH', '2', '2', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 10:59:01', null, null, '1', '2', '1', '功能按钮显示在列表工具栏+操作列', '0', 'BUTTON_SCOPE', '列表工具栏+操作栏', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY3BJQA66R2ACJW43SE0EXD', 'info', 'info', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 14:26:56', null, null, '1', '0', '1', '', '0', 'BUTTON_TYPE', 'info', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY3BKKUQLSJERSQCFXQ2B1Z', 'warning', 'warning', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 14:27:04', null, null, '1', '0', '1', '', '0', 'BUTTON_TYPE', 'warning', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY3BLEADAX4CR3Z7UYB2WXV', 'primary', 'primary', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 14:27:10', null, null, '1', '0', '1', '', '0', 'BUTTON_TYPE', 'primary', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY3BM7M7ORTVV8D1GK5YDWC', 'error', 'error', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 14:27:17', null, null, '1', '0', '1', '', '0', 'BUTTON_TYPE', 'error', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY9T74MB83WY3SQLQCCUUV3', 'F159', 'F159', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-04 10:23:01', '2024-12-04 10:23:39', null, '2', '159', '1', '由前端定义的预置功能，用于调用清空接口', '0', 'MENU_TYPE', '清空', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CY9TEOP8AYT88JT0COXZ0UY', 'F301', 'F301', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-04 10:24:06', '2024-12-04 10:39:37', null, '4', '301', '1', '由前端定义的预置功能，用于调用强制下线接口', '0', 'MENU_TYPE', '强制下线', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CYP4ZHMDHFQYVQIB8PNJ9RQ', '0', '0', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-06 09:27:42', null, null, '1', '0', '1', '', '0', 'LOG_METHOD', '不记录', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CYP51HASO7FOARNPCO1461I', '1', '1', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-06 09:27:59', null, null, '1', '1', '1', '', '0', 'LOG_METHOD', '文件记录', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CYP53OMGYQJ6444EQN1AM7M', '2', '2', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-06 09:28:18', null, null, '1', '2', '1', '', '0', 'LOG_METHOD', '数据库记录', '', '');
INSERT INTO `bxy_dict_data` VALUES ('03CYP55436ZTHTAWKW5D3N8PX', '3', '3', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-06 09:28:30', null, null, '1', '3', '1', '', '0', 'LOG_METHOD', '文件+数据库记录', '', '');

-- ----------------------------
-- Table structure for `bxy_dql`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_dql`;
CREATE TABLE `bxy_dql` (
  `id` varchar(255) NOT NULL,
  `create_by` varchar(255) NOT NULL,
  `update_by` varchar(255) DEFAULT NULL,
  `delete_by` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime DEFAULT NULL,
  `deleted_at` datetime DEFAULT NULL,
  `version` int(10) unsigned NOT NULL,
  `ord` int(10) unsigned NOT NULL,
  `status` varchar(255) NOT NULL,
  `remark` varchar(255) NOT NULL,
  `mcode` varchar(255) NOT NULL,
  `sign` varchar(255) NOT NULL,
  `dql` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_dql
-- ----------------------------
INSERT INTO `bxy_dql` VALUES ('03CX8CFU8JIVFQUQW8LCHX0PM', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-29 15:18:33', '2024-12-02 13:53:11', null, '9', '0', '1', '', '', 'BXY_LOGIN_LOG_LIST', 'select * from bxy_login_log where msg like CONCAT(\'%\', ?, \'%\')');
INSERT INTO `bxy_dql` VALUES ('03CXVKEFQ3WH75B5L8F16BMY0', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-02 14:37:57', '2024-12-02 17:42:38', null, '5', '0', '1', '获取所有可用的应用', '03cv1rqi0kw6qalxl661l1yg0', 'BXY_APP_ENABLED', 'select guid as value,app_name as label from bxy_app where status=\'1\'');
INSERT INTO `bxy_dql` VALUES ('03CXVZQGU1W1DLTKD34BOKNHA', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-02 15:56:27', '2024-12-02 17:42:20', null, '5', '0', '1', '获取指定应用下所有菜单目录，包括正常状态和停用状态', '03cv1rwvjw9hqcmsxy5axxzyn', 'BXY_MENU_C', 'select guid,pguid,mname from bxy_menu where (status=\'0\' or status=\'1\') and mtype=\'C\' and app=?');
INSERT INTO `bxy_dql` VALUES ('03CXW6OIAIKYQGGIIPKQYGC0G', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-02 16:32:00', '2024-12-02 17:41:58', null, '2', '0', '1', '获取所有数据字典分类', '03CWB2R35DGD9F20N3G28G6VA', 'BXY_DICT_TYPE', 'select guid,pguid,dname from bxy_dict where (status=\'0\' or status=\'1\')');
INSERT INTO `bxy_dql` VALUES ('03CY1KOR9RDF7ZEYJ15DAB9F6', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-03 09:05:09', '2024-12-03 09:08:11', null, '2', '0', '1', '获取指定类别下的所有字典数据', '03cv1sn88j69i3026fc36neap', 'BXY_DICT_DATA', 'select guid,pguid,att from bxy_dict_data where (status=\'0\' or status=\'1\') and dname=?');
INSERT INTO `bxy_dql` VALUES ('03CY1YGCMLI17OUF9NTY2LAGV', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-03 10:15:37', '2024-12-03 10:41:47', null, '3', '0', '1', '获取路由方式数据字典', '03cv1sn88j69i3026fc36neap', 'BXY_DICT_ROUTE', 'select guid+0 as value,att as label from bxy_dict_data where (status=\'0\' or status=\'1\') and dname=\'ROUTE_MODEL\'');
INSERT INTO `bxy_dql` VALUES ('03CY20KMMRRY8BFNVOWSGOO2D', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-03 10:26:28', '2024-12-03 10:28:43', null, '2', '0', '1', '获取HTTP请求类型数据字典', '03cv1sn88j69i3026fc36neap', 'BXY_DICT_HTTP', 'select guid as value,att as label from bxy_dict_data where (status=\'0\' or status=\'1\') and dname=\'HTTP_METHOD\'');
INSERT INTO `bxy_dql` VALUES ('03CY224UQ47TT5T3Y2WAICGDD', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-03 10:34:28', '2024-12-03 10:42:25', null, '2', '0', '1', '获取打开方式数据字典', '03cv1sn88j69i3026fc36neap', 'BXY_DICT_OPEN', 'select guid+0 as value, att as label from bxy_dict_data where (status=\'0\' or status=\'1\') and dname=\'OPEN_MODEL\'');
INSERT INTO `bxy_dql` VALUES ('03CY25U6NLY4VDN2DW6KJJ723', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-03 10:53:25', '2024-12-03 10:54:42', null, '2', '0', '1', '获取功能类型数据字典', '03cv1sn88j69i3026fc36neap', 'BXY_DICT_FN', 'select guid as value, att as label from bxy_dict_data where (status=\'0\' or status=\'1\') and dname=\'MENU_TYPE\'	');
INSERT INTO `bxy_dql` VALUES ('03CY273K7Y809F1UYUI1RFT50', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-03 10:59:52', '2024-12-03 14:30:12', null, '2', '0', '1', '获取功能按钮显示区域数据字典', '03cv1sn88j69i3026fc36neap', 'BXY_DICT_BUTTON_SCOPE', 'select guid+0 as value, att as label from bxy_dict_data where (status=\'0\' or status=\'1\') and dname=\'BUTTON_SCOPE\'');
INSERT INTO `bxy_dql` VALUES ('03CY2BDBRBQ6PDGUOI9J54A4I', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 11:21:44', null, null, '1', '0', '1', '获取组织部门数据字典', '03cv1s494q7oqwk231b3ywyde', 'BXY_ORG_TREE', 'select guid,pguid,oname from bxy_org where (status=\'0\' or status=\'1\')');
INSERT INTO `bxy_dql` VALUES ('03CY2C7ILRVTMJKD8RJ00UY98', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 11:26:02', null, null, '1', '0', '1', '获取角色数据字典', '03cv1s4ummhh1caqebzlelian', 'BXY_ROLE_TREE', 'select guid,pguid,rname from bxy_role where (status=\'0\' or status=\'1\')');
INSERT INTO `bxy_dql` VALUES ('03CY2CA4H0GTMCVL35OKW8IF4', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 11:26:24', null, null, '1', '0', '1', '获取岗位数据字典', '03cv1s5idb4cpvqshtj7vibtt', 'BXY_POST_TREE', 'select guid,pguid,pname from bxy_post where (status=\'0\' or status=\'1\')');
INSERT INTO `bxy_dql` VALUES ('03CY2CC8A3OPW1MC6E32UH25H', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 11:26:42', null, null, '1', '0', '1', '获取职务数据字典', '03cv1s67tys8b1rp4i64n4pzy', 'BXY_DUTY_TREE', 'select guid,pguid,dname from bxy_duty where (status=\'0\' or status=\'1\')');
INSERT INTO `bxy_dql` VALUES ('03CY3C3XQKGP3YBPFI85QRNNL', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-03 14:29:49', '2024-12-04 14:48:49', null, '3', '0', '1', '获取功能按钮样式数据字典', '03cv1sn88j69i3026fc36neap', 'BXY_DICT_BUTTON_TYPE', 'select guid as value, att as label from bxy_dict_data where (status=\'0\' or status=\'1\') and dname=\'BUTTON_TYPE\'');
INSERT INTO `bxy_dql` VALUES ('03CY3CU5XNRAARGHTJJ2X7633', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-03 14:33:32', '2024-12-03 15:02:31', null, '3', '0', '1', '获取授权类型数据字典', '03cv1sn88j69i3026fc36neap', 'BXY_DICT_AUTH_TYPE', 'select guid+0 as value, att as label from bxy_dict_data where (status=\'0\' or status=\'1\') and dname=\'AUTH_TYPE\'');
INSERT INTO `bxy_dql` VALUES ('03CY3CW9RO8JH2AECMZCD89XW', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-03 14:33:50', '2024-12-03 15:02:49', null, '2', '0', '1', '获取授权方式数据字典', '03cv1sn88j69i3026fc36neap', 'BXY_DICT_AUTH_METHOD', 'select guid+0 as value, att as label from bxy_dict_data where (status=\'0\' or status=\'1\') and dname=\'AUTH_METHOD\'');
INSERT INTO `bxy_dql` VALUES ('03CY3ERJFSU3BQMZ0HKGZBNH4', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 14:43:24', null, null, '1', '0', '1', '获取所有用户', '03cv1s3mjc4w2ef857yjrqhng', 'BXY_USER_UID', 'select u_id as value,uname as label from bxy_user where (status=\'0\' or status=\'1\')');
INSERT INTO `bxy_dql` VALUES ('03CYBVXBVN0T69KYNH2LUP40X', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-04 16:45:34', null, null, '1', '0', '1', '获取模块功能树', '03cv1rwvjw9hqcmsxy5axxzyn', 'BXY_MENU_FN', 'select guid,pguid,mname from bxy_menu where (status=\'0\' or status=\'1\')');
INSERT INTO `bxy_dql` VALUES ('03CYHKC1P51GS8PFZV87RW2TQ', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-05 10:11:58', null, null, '1', '0', '1', '获取指定菜单模块下配置的所有附件类别', '03cv1shm07ak44u24iz88yegx', 'BXY_ADTION_TYPE', 'select guid,pguid,exname from bxy_adtion_ex where mcode=?');
INSERT INTO `bxy_dql` VALUES ('03CYP5C50EED1OO0170YI95X5', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-06 09:29:30', '2024-12-06 09:32:11', null, '3', '0', '1', '', '03cv1sn88j69i3026fc36neap', 'BXY_DICT_LOG_METHOD', 'select guid as value,att as label from bxy_dict_data where (status=\'0\' or status=\'1\') and dname=\'LOG_METHOD\'');

-- ----------------------------
-- Table structure for `bxy_duty`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_duty`;
CREATE TABLE `bxy_duty` (
  `id` varchar(255) NOT NULL,
  `guid` varchar(255) NOT NULL,
  `pguid` varchar(255) NOT NULL,
  `create_by` varchar(255) NOT NULL,
  `update_by` varchar(255) DEFAULT NULL,
  `delete_by` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime DEFAULT NULL,
  `deleted_at` datetime DEFAULT NULL,
  `version` int(10) unsigned NOT NULL,
  `ord` int(10) unsigned NOT NULL,
  `status` varchar(255) NOT NULL,
  `remark` varchar(255) NOT NULL,
  `dname` varchar(255) NOT NULL,
  `att` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_duty
-- ----------------------------
INSERT INTO `bxy_duty` VALUES ('03CWQT2VHNYDXELBNJ8ZQ800V', '03CWQT2VHNYDXELBNJBR1P4UU', '03CWQT2VHNYDXELBNJBR1P4UU', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-27 09:26:32', '2024-12-04 14:29:19', null, '3', '0', '1', '', '研发经理', '技术岗');
INSERT INTO `bxy_duty` VALUES ('03CYB4IVBT5BBZZOIZJN8VJJR', '03CYB4IVBT5BBZZOIZKYGA1CX', '03CYB4IVBT5BBZZOIZKYGA1CX', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-04 14:25:18', '2024-12-04 14:29:35', null, '2', '0', '1', '', '实施经理', '技术岗');

-- ----------------------------
-- Table structure for `bxy_job`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_job`;
CREATE TABLE `bxy_job` (
  `id` varchar(255) NOT NULL,
  `guid` varchar(255) NOT NULL,
  `create_by` varchar(255) NOT NULL,
  `update_by` varchar(255) DEFAULT NULL,
  `delete_by` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime DEFAULT NULL,
  `deleted_at` datetime DEFAULT NULL,
  `version` int(10) unsigned NOT NULL,
  `ord` int(10) unsigned NOT NULL,
  `status` varchar(255) NOT NULL,
  `remark` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_job
-- ----------------------------

-- ----------------------------
-- Table structure for `bxy_job_log`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_job_log`;
CREATE TABLE `bxy_job_log` (
  `id` varchar(255) NOT NULL,
  `job_id` varchar(255) NOT NULL,
  `lot_id` bigint(20) NOT NULL,
  `lot_order` bigint(20) NOT NULL,
  `job_name` varchar(255) NOT NULL,
  `job_group` varchar(255) NOT NULL,
  `invoke_target` varchar(255) NOT NULL,
  `job_params` varchar(255) DEFAULT NULL,
  `job_message` varchar(255) DEFAULT NULL,
  `status` varchar(255) NOT NULL,
  `exception_info` varchar(255) DEFAULT NULL,
  `is_once` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `elapsed_time` bigint(20) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_job_log
-- ----------------------------

-- ----------------------------
-- Table structure for `bxy_login_log`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_login_log`;
CREATE TABLE `bxy_login_log` (
  `id` varchar(255) NOT NULL,
  `u_id` varchar(255) NOT NULL,
  `net` varchar(255) NOT NULL,
  `ip` varchar(255) NOT NULL,
  `login_location` varchar(255) NOT NULL,
  `browser` varchar(255) NOT NULL,
  `os` varchar(255) NOT NULL,
  `device` varchar(255) NOT NULL,
  `status` varchar(255) NOT NULL,
  `msg` varchar(255) NOT NULL,
  `login_time` datetime NOT NULL,
  `module` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_login_log
-- ----------------------------
INSERT INTO `bxy_login_log` VALUES ('03cv1r0f63rb1v3me9tjsrd41', 'admin', '电信', '114.222.36.78', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-19 13:54:19', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cv2eguverufmp67olzajcjn', 'admin', '电信', '114.222.36.78', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-19 15:54:24', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cv2en649woi00l3gkr6ahpl', 'admin', '电信', '114.222.36.78', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-19 15:55:18', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cv2f5impxw15skn1oj6g2wr', 'admin', '电信', '114.222.36.78', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-19 15:57:54', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cv2ftkyk3jdjinkhk7zblqz', 'admin', '电信', '114.222.36.78', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-19 16:01:20', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cv2g1lrlaxfvfbtbmiv7fj8', 'admin', '电信', '114.222.36.78', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-19 16:02:28', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cv2g9znxrrutm7x6cn4f6hw', 'admin', '电信', '114.222.36.78', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-19 16:03:40', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cv2gevvn2eivb0rph41bjmx', 'admin', '电信', '114.222.36.78', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-19 16:04:21', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cv2gipjbjt2jsx2zz3kmogs', 'admin', '电信', '114.222.36.78', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-19 16:04:54', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cv2h57ux0t5um1opyj6mj6p', 'admin', '电信', '114.222.36.78', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-19 16:08:06', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cv86ds6qpgjftxi6j1214q7', 'admin', '电信', '114.222.131.131', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-20 09:38:45', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cv86mb4ar2trqonbjvyv7b2', 'admin', '电信', '114.222.131.131', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-20 09:39:57', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cv86rrdwcsutvqxh2xk3mah', 'admin', '电信', '114.222.131.131', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-20 09:40:44', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cv8ljul8zyarwnbxdpjn1in', 'admin', '电信', '114.222.131.131', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-20 10:56:24', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cv8sb1lmg51cbpteewd3mm0', 'admin', '电信', '114.222.131.131', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-20 11:30:58', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cv8wmltbxwxni7vgqau2v10', 'admin', '电信', '114.222.131.131', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-20 11:53:06', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cv8zhubcc9pw69f5zejpc13', 'admin', '电信', '114.222.131.131', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-20 12:07:46', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cv8zmi6p49rvtnpi8k81uha', 'admin', '电信', '114.222.131.131', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-20 12:08:26', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cv93eu4le4r0k8jafs55j5w', 'admin', '电信', '114.222.131.131', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-20 12:27:49', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cv9i6v7x9i3yjx1xaeggvv6', 'admin', '电信', '114.222.131.131', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-20 13:43:29', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvalbbaxfkopdpml132nysc', 'admin', '电信', '114.222.131.131', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-20 17:03:45', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvfqf29nzcf1ocwozkj7arl', 'admin', '电信', '114.221.243.8', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-21 08:51:20', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvfqjyb0itbrp89p8xc586f', 'admin', '电信', '114.221.243.8', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-21 08:52:01', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvfr84jlu0bua6zcnpqx5hm', 'admin', '电信', '114.221.243.8', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-21 08:55:28', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvfuy52qn7vif640it6vs31', 'admin', '电信', '114.221.243.8', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-21 09:14:31', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvfy99x5zdsuc4kx6adyh9u', 'admin', '电信', '114.221.243.8', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-21 09:31:28', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvga8r5rv9rdvyatmhmtcj4', 'admin', '电信', '114.221.243.8', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-21 10:32:49', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvgud2xn0khe5jlv5wbu6g8', 'admin', '电信', '114.221.243.8', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-21 12:15:49', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvhj0bv4ktig8cjbepzd6tw', 'admin', '电信', '114.221.243.8', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '0', '验证码错误', '2024-11-21 14:21:59', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvhj1hprui28u28qahw3urs', 'admin', '电信', '114.221.243.8', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-21 14:22:09', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvhlw14p5dkd4878gx8h9qh', 'admin', '电信', '114.221.243.8', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-21 14:36:43', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvhmca53h7yxze3i0z7dwnl', 'admin', '电信', '114.221.243.8', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-21 14:39:02', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvhnpn68lb7fc1y7nn9mibw', 'admin', '电信', '114.221.243.8', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-21 14:46:03', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvhx13zxy9l4z5hu2di5fg6', 'admin', '电信', '114.221.243.8', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-21 15:33:45', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvhx5lleppb2zush68xfhor', 'admin', '电信', '114.221.243.8', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-21 15:34:24', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvhxfukqc9ti1a3vsh9ru85', 'admin', '电信', '114.221.243.8', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-21 15:35:51', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvhzvnx2f7nq6jnrsfkic7t', 'admin', '电信', '114.221.243.8', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-21 15:48:20', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvi3dhmm0mp99mwusaf6wts', 'admin', '电信', '114.221.243.8', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-21 16:06:14', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvi50sfp3ud065vossp2mgb', 'admin', '电信', '114.221.243.8', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-21 16:14:40', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvi5uo1az9aq3ebv996k9w1', 'admin', '电信', '114.221.243.8', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-21 16:18:55', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvi657ct95xjuvm9qi45tx7', 'admin', '电信', '114.221.243.8', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-21 16:20:25', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvp4j5yv4ew5lt4p668t7gz', 'admin', '电信', '114.222.39.126', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-22 13:42:11', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvp4ns2j3un0fgqwcbw0yjq', 'admin', '电信', '114.222.39.126', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-22 13:42:50', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvpbkjmxwgzrl3i4ydme4a7', 'admin', '电信', '114.222.39.126', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-22 14:18:13', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvpboff5cqg29a45tqafpak', 'admin', '电信', '114.222.39.126', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-22 14:18:46', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03cvpbph5lsuj75pjr4esk2xl', 'admin', '电信', '114.222.39.126', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-22 14:18:55', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03CWB393DLNPW8N7U9XFG0GAD', 'admin', '电信', '117.89.216.20', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-25 09:09:59', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03CWB3Y8WUTR72X5YU8TO0MEW', 'admin', '电信', '117.89.216.20', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-25 09:13:34', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03CWDEC6FTIMYHF9TYLEFMZ4C', 'admin', '电信', '117.89.216.20', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-25 16:15:19', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03CWS5TKIRFBQUEUX0D6BA70E', 'admin', '电信', '114.222.172.113', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-11-27 13:36:03', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03CY9W4FQG2A3WFUB6K555Z01', 'admin', '电信', '117.89.216.20', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-12-04 10:38:00', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03CYA02VPO27827A1BDWBYNAU', 'admin', '电信', '117.89.216.20', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-12-04 10:58:15', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03CYQ1J6RXRRII8PR4UCV4MMK', 'admin', '电信', '114.222.172.54', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-12-06 12:14:19', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03CYQ9LUICAOQ18CJ590R0P6Z', 'admin', '电信', '114.222.172.54', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-12-06 12:55:39', '用户模块');
INSERT INTO `bxy_login_log` VALUES ('03CYQB7B602SU5K0ZZSUDCQQH', 'admin', '电信', '114.222.172.54', '江苏省南京市', 'Edge 131', 'Windows 10', 'Other', '1', '登录成功', '2024-12-06 13:03:49', '用户模块');

-- ----------------------------
-- Table structure for `bxy_md`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_md`;
CREATE TABLE `bxy_md` (
  `id` varchar(255) NOT NULL,
  `create_by` varchar(255) NOT NULL,
  `update_by` varchar(255) DEFAULT NULL,
  `delete_by` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime DEFAULT NULL,
  `deleted_at` datetime DEFAULT NULL,
  `version` int(10) unsigned NOT NULL,
  `ord` int(10) unsigned NOT NULL,
  `status` varchar(255) NOT NULL,
  `remark` varchar(255) NOT NULL,
  `mcode` varchar(255) NOT NULL,
  `dcode` varbinary(255) DEFAULT NULL,
  `m_fields` varchar(255) NOT NULL,
  `d_fields` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_md
-- ----------------------------
INSERT INTO `bxy_md` VALUES ('03CYHTSVVSR1MT4GO02UGQQP7', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-05 11:00:26', null, null, '1', '1', '1', '', '03cv1rqi0kw6qalxl661l1yg0', 0x30336376317277766A77396871636D737879356178787A796E, 'guid', 'app');

-- ----------------------------
-- Table structure for `bxy_menu`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_menu`;
CREATE TABLE `bxy_menu` (
  `id` varchar(255) NOT NULL,
  `guid` varchar(255) NOT NULL,
  `pguid` varchar(255) NOT NULL,
  `create_by` varchar(255) NOT NULL,
  `update_by` varchar(255) DEFAULT NULL,
  `delete_by` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime DEFAULT NULL,
  `deleted_at` datetime DEFAULT NULL,
  `version` int(10) unsigned NOT NULL,
  `ord` int(10) unsigned NOT NULL,
  `status` varchar(255) NOT NULL,
  `remark` varchar(255) NOT NULL,
  `app` varchar(255) NOT NULL,
  `mname` varchar(255) NOT NULL,
  `mtype` varchar(255) NOT NULL,
  `uio` tinyint(3) unsigned DEFAULT NULL,
  `path` varchar(255) NOT NULL,
  `api` varchar(255) NOT NULL,
  `method` varchar(50) DEFAULT NULL,
  `opt` tinyint(3) unsigned NOT NULL,
  `alias` varchar(255) NOT NULL,
  `tbl` varchar(255) DEFAULT NULL,
  `query` varchar(255) DEFAULT NULL,
  `qscript` varchar(255) DEFAULT NULL,
  `cols` varchar(255) DEFAULT NULL,
  `cscript` varchar(255) DEFAULT NULL,
  `icon` varchar(255) NOT NULL,
  `style` varchar(255) NOT NULL,
  `show` tinyint(3) unsigned NOT NULL,
  `comp` varchar(255) DEFAULT NULL,
  `visible` varchar(255) NOT NULL,
  `is_cache` varchar(255) NOT NULL,
  `log_method` varchar(255) NOT NULL,
  `data_cache_method` varchar(255) NOT NULL,
  `data_scope` varchar(255) NOT NULL,
  `i18n` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_menu
-- ----------------------------
INSERT INTO `bxy_menu` VALUES ('03cv1oxlksezymqizbt9oi62t', '03cv1oxlksezymqizbw2d0psz', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-19 13:43:41', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '开发后台', 'C', '0', '', '', '', '0', '', '', '', '', '', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cv1rdvewmo40c88vht9clpc', '03cv1rdvewmo40c88vj2ygi8a', '03cv1rdvewmo40c88vj2ygi8a', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-19 13:56:14', '2024-11-26 15:22:04', null, '2', '401', '1', '', '03ctywcl3sjqt4l75nzfh495n', '流程引擎', 'C', '0', '', '', '', '0', '', '', '', '', '', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cv1rqi0kw6qalxl64ujd9jh', '03cv1rqi0kw6qalxl661l1yg0', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-19 13:58:02', '2024-12-06 09:34:12', null, '5', '101', '1', '', '03ctywcl3sjqt4l75nzfh495n', '应用管理', 'C', '0', '@bpm/views/TableView', '/api/v1/core/app/find_all', 'GET', '0', '开发后台 > 应用管理', 'D', '@bpm/views/admin/app/QueryV', '', '@bpm/views/admin/list/app_list', '', '', '', '0', '', '0', '', '2', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cv1ru8zt9ssjw4zmxbmkxhg', '03cv1ru8zt9ssjw4zmzfvv9u4', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-19 13:58:34', '2024-12-05 12:34:40', null, '3', '101', '1', '', '03ctywcl3sjqt4l75nzfh495n', '应用授权', 'C', '0', '@bpm/views/TableView', '/api/v1/core/auth/app/find_all', 'GET', '0', '', 'D', '@bpm/views/admin/app/QueryAuthV', '', '@bpm/views/admin/list/app_auth_list', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cv1rwvjw9hqcmsxy2ew2mrt', '03cv1rwvjw9hqcmsxy5axxzyn', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-19 13:58:56', '2024-12-05 12:46:30', null, '5', '102', '1', '', '03ctywcl3sjqt4l75nzfh495n', '菜单管理', 'C', '0', '@bpm/views/TableView', '/api/v1/core/menu/find_menu_all', 'GET', '0', '开发后台 > 菜单管理', 'D', '@bpm/views/admin/menu/QueryMenuV', '', '@bpm/views/admin/list/menu_list', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cv1s3mjc4w2ef857xputsft', '03cv1s3mjc4w2ef857yjrqhng', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-19 13:59:54', '2024-12-05 13:19:37', null, '3', '200', '1', '', '03ctywcl3sjqt4l75nzfh495n', '用户管理', 'C', '0', '@bpm/views/TableView', '/api/v1/core/user/find_all', 'GET', '0', '', 'D', '@bpm/views/admin/user/QueryV', '', '@bpm/views/admin/list/user_list', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cv1s494q7oqwk2319z2cnux', '03cv1s494q7oqwk231b3ywyde', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-19 13:59:59', '2024-12-05 13:14:07', null, '4', '104', '1', '', '03ctywcl3sjqt4l75nzfh495n', '部门管理', 'C', '0', '@bpm/views/TableView', '/api/v1/core/org/find_all', 'GET', '0', '', 'D', '@bpm/views/admin/org/QueryV', '', '@bpm/views/admin/list/org_list', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cv1s4ummhh1caqebxc5q64q', '03cv1s4ummhh1caqebzlelian', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-19 14:00:04', '2024-12-05 13:13:56', null, '5', '105', '1', '', '03ctywcl3sjqt4l75nzfh495n', '角色管理', 'C', '0', '@bpm/views/TableView', '/api/v1/core/role/find_all', 'GET', '0', '', 'D', '@bpm/views/admin/role/QueryV', '', '@bpm/views/admin/list/role_list', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cv1s5idb4cpvqshtfwk5odb', '03cv1s5idb4cpvqshtj7vibtt', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-19 14:00:10', '2024-12-05 13:13:45', null, '4', '105', '1', '', '03ctywcl3sjqt4l75nzfh495n', '岗位管理', 'C', '0', '@bpm/views/TableView', '/api/v1/core/post/find_all', 'GET', '0', '', 'D', '@bpm/views/admin/post/QueryV', '', '@bpm/views/admin/list/post_list', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cv1s67tys8b1rp4i509e3fy', '03cv1s67tys8b1rp4i64n4pzy', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-19 14:00:16', '2024-12-05 13:13:30', null, '4', '107', '1', '', '03ctywcl3sjqt4l75nzfh495n', '职务管理', 'C', '0', '@bpm/views/TableView', '/api/v1/core/duty/find_all', 'GET', '0', '', 'D', '@bpm/views/admin/duty/QueryV', '', '@bpm/views/admin/list/duty_list', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cv1s9wen3gcoh0qp53gjkg6', '03cv1s9wen3gcoh0qp8jmyuj4', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-19 14:00:47', '2024-12-05 14:32:11', null, '5', '212', '1', '', '03ctywcl3sjqt4l75nzfh495n', '列级授权', 'C', '0', '@bpm/views/TableView', '/api/v1/core/auth/col/find_all', 'GET', '0', '开发后台 > 列级授权', '', '@bpm/views/admin/auth/QueryColV', '', '@bpm/views/admin/list/col_auth_list', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cv1sahb19swyi9afbg4ecxi', '03cv1sahb19swyi9afdnujp36', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-19 14:00:52', '2024-12-05 14:22:09', null, '4', '211', '1', '', '03ctywcl3sjqt4l75nzfh495n', '行级授权', 'C', '0', '@bpm/views/TableView', '/api/v1/core/auth/row/find_all', 'GET', '0', '开发后台 > 行级授权', '', '@bpm/views/admin/auth/QueryRowV', '', '@bpm/views/admin/list/row_auth_list', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cv1sb745kzhmvs78j2dwcfs', '03cv1sb745kzhmvs78jl9xm6t', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-19 14:00:58', '2024-12-05 13:57:01', null, '5', '210', '1', '', '03ctywcl3sjqt4l75nzfh495n', '功能授权', 'C', '0', '@bpm/views/TableView', '/api/v1/core/auth/fn/find_all', 'GET', '0', '', '', '@bpm/views/admin/auth/QueryFnV', '', '@bpm/views/admin/list/user_auth_list', '', '', '开发后台 > 功能授权', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cv1sd1fzc27jtsnpg6wbn1o', '03cv1sd1fzc27jtsnpiu7xvy5', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-19 14:01:14', '2024-12-05 15:35:24', null, '3', '402', '1', '', '03ctywcl3sjqt4l75nzfh495n', '登录日志', 'C', '0', '@bpm/views/TableView', '/api/v1/core/log/login/find_all', 'GET', '0', '', 'D', '@bpm/views/admin/log/QueryLoginV', '', '@bpm/views/admin/list/login_log_list', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cv1sdp8oxkloyod96k51xix', '03cv1sdp8oxkloyod99aio7rk', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-19 14:01:20', '2024-11-26 15:23:30', null, '2', '403', '1', '', '03ctywcl3sjqt4l75nzfh495n', '操作日志', 'C', '0', '@bpm/views/TableView', '/api/v1/core/log/oper/find_all', 'GET', '0', '', 'D', '', '', '@bpm/views/admin/list/oper_log_list', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cv1sfv869o7zrt8lryondz1', '03cv1sfv869o7zrt8ls9xqxbm', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-19 14:01:38', '2024-12-05 15:31:00', null, '5', '400', '1', '', '03ctywcl3sjqt4l75nzfh495n', '在线用户', 'C', '0', '@bpm/views/TableView', '/api/v1/core/online/find_all', 'GET', '0', '开发后台 > 在线用户', '', '@bpm/views/admin/user/QueryOnlineV', '', '@bpm/views/admin/list/online_list', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cv1sh19h7eguyipnmfe496i', '03cv1sh19h7eguyipnnj1s9xc', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-19 14:01:48', '2024-12-05 14:55:36', null, '5', '321', '1', '', '03ctywcl3sjqt4l75nzfh495n', '附件数据', 'C', '0', '@bpm/views/TableView', '/api/v1/core/adtion/data/find_all', 'GET', '0', '开发后台 > 附件数据', '', '@bpm/views/admin/adtion/QueryV', '', '@bpm/views/admin/list/adtion_list', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cv1shm07ak44u24iwwl25d9', '03cv1shm07ak44u24iz88yegx', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-19 14:01:53', '2024-12-05 14:49:48', null, '4', '320', '1', '', '03ctywcl3sjqt4l75nzfh495n', '附件类别', 'C', '0', '@bpm/views/TableView', '/api/v1/core/adtion/type/find_all', 'GET', '0', '', '', '@bpm/views/admin/adtion/QueryExV', '', '@bpm/views/admin/list/adtion_ex_list', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cv1sn88j69i3026f9y50wnk', '03cv1sn88j69i3026fc36neap', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-19 14:02:41', '2024-12-05 15:04:50', null, '4', '311', '1', '', '03ctywcl3sjqt4l75nzfh495n', '字典数据', 'C', '0', '@bpm/views/TableView', '/api/v1/core/dict/data/find_all', 'GET', '0', '', 'D', '@bpm/views/admin/dict/QueryDataV', '', '@bpm/views/admin/list/dict_data_list', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cv1st6s0hh0xnjb544wx3an', '03cv1st6s0hh0xnjb56ga7qtb', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-19 14:03:32', '2024-12-05 15:24:47', null, '5', '331', '1', '', '03ctywcl3sjqt4l75nzfh495n', '树型关联', 'C', '0', '@bpm/views/TableView', '/api/v1/core/tree/find_all', 'GET', '0', '开发后台 > 树型关联', '', '@bpm/views/admin/tree/QueryV', '', '@bpm/views/admin/list/tree_list', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cv1su0vikj86xs1yuh9pb8f', '03cv1su0vikj86xs1ywslvkge', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-19 14:03:39', '2024-12-05 15:15:33', null, '4', '330', '1', '', '03ctywcl3sjqt4l75nzfh495n', '主从关联', 'C', '0', '@bpm/views/TableView', '/api/v1/core/md/find_all', 'GET', '0', '', '', '@bpm/views/admin/md/QueryV', '', '@bpm/views/admin/list/md_list', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cv1sy4mem4a1cgfiequnmav', '03cv1sy4mem4a1cgfigq91zds', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-19 14:04:14', '2024-12-05 15:01:13', null, '5', '220', '1', '', '03ctywcl3sjqt4l75nzfh495n', '用户接口', 'C', '0', '@bpm/views/TableView', '/api/v1/core/auth/api/find_all', 'GET', '0', '开发后台 > 用户接口', '', '@bpm/views/admin/user/QueryApiV', '', '@bpm/views/admin/list/user_api_list', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cvnlxko5wzbzqcyqtezgvmf', '03cvnlxko5wzbzqcyqvo0jk86', '03cv1rqi0kw6qalxl661l1yg0', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-22 09:02:41', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '新增', 'F110', '0', '@bpm/views/admin/app/FormV', '/api/v1/core/app/add', 'POST', '0', '应用管理 > 新增', '', '', '', '', '', '', 'info', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cvnm06fgn5w9u4u2j7n1we6', '03cvnm06fgn5w9u4u2ldxptk5', '03cv1rqi0kw6qalxl661l1yg0', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-22 09:03:03', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '编辑', 'F120', '0', '@bpm/views/admin/app/FormV', '/api/v1/core/app/edit', 'PUT', '0', '应用管理 > 编辑', '', '', '', '', '', '', 'warning', '1', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cvnm1h0xtupcrnnq9yw21f5', '03cvnm1h0xtupcrnnqbwee6ik', '03cv1rqi0kw6qalxl661l1yg0', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-22 09:03:14', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '浏览', 'F130', '0', '@bpm/views/admin/app/FormV', '', 'GET', '0', '应用管理 > 浏览', '', '', '', '', '', '', 'primary', '1', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03cvnm2kirqkeqsbwqi718on6', '03cvnm2kirqkeqsbwqik29ges', '03cv1rqi0kw6qalxl661l1yg0', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-22 09:03:23', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '删除', 'F140', '0', '', '', 'PUT', '0', '应用管理 > 软删除', '', '', '', '', '', '', 'error', '2', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWB2R35DGD9F20N3DZFHD74', '03CWB2R35DGD9F20N3G28G6VA', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-25 09:07:25', '2024-12-05 14:37:38', null, '4', '310', '1', '', '03ctywcl3sjqt4l75nzfh495n', '字典类别', 'C', '0', '@bpm/views/TableView', '/api/v1/core/dict/type/find_all', 'GET', '0', '开发后台 > 字典类别', '', '@bpm/views/admin/dict/QueryV', '', '@bpm/views/admin/list/dict_list', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWB99T9K0XTPUHOYYHV4OS0', '03CWB99T9K0XTPUHOYZNECIPO', '03CWB2R35DGD9F20N3G28G6VA', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-25 09:40:48', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '新增', 'F110', '0', '@bpm/views/admin/dict/FormV', '/api/v1/core/dict/type/add', 'POST', '0', '字典类别 > 新增', '', '', '', '', '', '', 'info', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWB9C3BIDZMHDJFPNABZNXB', '03CWB9C3BIDZMHDJFPPE2I6NO', '03CWB2R35DGD9F20N3G28G6VA', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-25 09:41:08', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '编辑', 'F120', '0', '@bpm/views/admin/dict/FormV', '/api/v1/core/dict/type/edit', 'PUT', '0', '字典类别 > 编辑', '', '', '', '', '', '', 'warning', '1', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWB9D88A0DD9LMO10XT85P3', '03CWB9D88A0DD9LMO1282RBPD', '03CWB2R35DGD9F20N3G28G6VA', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-25 09:41:17', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '浏览', 'F130', '0', '@bpm/views/admin/dict/FormV', '', 'GET', '0', '字典类别 > 浏览', '', '', '', '', '', '', 'primary', '1', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWB9EM9ITXWIH2TXX2OHGYF', '03CWB9EM9ITXWIH2TY0A9013R', '03CWB2R35DGD9F20N3G28G6VA', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-25 09:41:29', '2024-12-05 09:47:20', null, '2', '4', '1', '', '03ctywcl3sjqt4l75nzfh495n', '删除', 'F140', '0', '', '/api/v1/core/dict/type/remove', 'PUT', '0', '字典类别 > 删除', '', '', '', '', '', '', 'error', '2', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWIZU6DOCGUT1C9MQZVV1MX', '03CWIZU6DOCGUT1C9MRNOFR8J', '03cv1ru8zt9ssjw4zmzfvv9u4', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-26 09:26:49', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '删除', 'F150', '0', '', '/api/v1/core/auth/app/delete', 'DELETE', '0', '应用授权 > 删除', '', '', '', '', '', '', 'error', '2', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWK4GORUP6VGLE26VAYELQG', '03CWK4GOS061Y8AF149QKQPHQ', '03cv1rwvjw9hqcmsxy5axxzyn', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-26 12:54:47', null, null, '1', '10', '1', '', '03ctywcl3sjqt4l75nzfh495n', '新增', 'F110', '0', '@bpm/views/admin/menu/MenuFormV', '/api/v1/core/menu/add', 'POST', '0', '菜单管理 > 新增', '', '', '', '', '', '', 'info', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWK4ILLVEQXXZHI0SH5IA8S', '03CWK4ILLVEQXXZHI0TPMFNL8', '03cv1rwvjw9hqcmsxy5axxzyn', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-26 12:55:03', null, null, '1', '11', '1', '', '03ctywcl3sjqt4l75nzfh495n', '编辑', 'F120', '0', '@bpm/views/admin/menu/MenuFormV', '/api/v1/core/menu/edit', 'PUT', '0', '菜单管理 > 编辑', '', '', '', '', '', '', 'warning', '1', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWK4M7CKOFEO25VXSPPFFL2', '03CWK4M7CKOFEO25VXU36HOHW', '03cv1rwvjw9hqcmsxy5axxzyn', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-26 12:55:34', null, null, '1', '12', '1', '', '03ctywcl3sjqt4l75nzfh495n', '浏览', 'F130', '0', '@bpm/views/admin/menu/MenuFormV', '', 'GET', '0', '菜单管理 > 浏览', '', '', '', '', '', '', 'primary', '1', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWK4OJLTI24K0CPJ8PKPDXV', '03CWK4OJLTI24K0CPJCK4E48U', '03cv1rwvjw9hqcmsxy5axxzyn', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-26 12:55:54', null, null, '1', '13', '1', '', '03ctywcl3sjqt4l75nzfh495n', '删除', 'F140', '0', '', '/api/v1/core/menu/remove', 'DELETE', '0', '菜单管理 > 删除', '', '', '', '', '', '', 'error', '2', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWKMPFI3Z7UGDXEAR4LVP0G', '03CWKMPFI3Z7UGDXEAU21OVOK', '03cv1s3mjc4w2ef857yjrqhng', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-26 14:28:11', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '新增', 'F110', '0', '@bpm/views/admin/user/FormV', '/api/v1/core/user/add', 'POST', '0', '用户管理 > 新增', '', '', '', '', '', '', 'info', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWKMSU8ZPES9N785KHGA7NI', '03CWKMSU8ZPES9N785M7XIB4G', '03cv1s3mjc4w2ef857yjrqhng', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-26 14:28:39', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '编辑', 'F120', '0', '@bpm/views/admin/user/FormV', '/api/v1/core/user/edit', 'PUT', '0', '用户管理 > 编辑', '', '', '', '', '', '', 'warning', '1', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWKMVXMGMXJW2BYSDQ4ZIFB', '03CWKMVXMGMXJW2BYSEMG0XC7', '03cv1s3mjc4w2ef857yjrqhng', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-26 14:29:06', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '浏览', 'F130', '0', '@bpm/views/admin/user/FormV', '', 'GET', '0', '用户管理 > 浏览', '', '', '', '', '', '', 'primary', '1', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWKN2U26TB0AFNJU9MACRNK', '03CWKN2U26TB0AFNJUAS5FXQQ', '03cv1s3mjc4w2ef857yjrqhng', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-26 14:30:05', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '删除', 'F140', '0', '', '/api/v1/core/user/remove', 'PUT', '0', '用户管理 > 删除', '', '', '', '', '', '', 'error', '2', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWKT9HR52RTASIY8BEPR223', '03CWKT9HR52RTASIY8C3TI560', '03cv1rwvjw9hqcmsxy5axxzyn', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-26 15:01:44', null, null, '1', '1', '1', '', '03ctywcl3sjqt4l75nzfh495n', '功能', 'F100', '0', '@bpm/views/admin/menu/FnForm', '/api/v1/core/menu/add', 'POST', '0', '菜单管理 > 功能', '', '', '', '', '', '', 'info', '1', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWL5I20FILTIU1KQZ7ECS6O', '03CWL5I20FILTIU1KR1XSVGYD', '03cv1s494q7oqwk231b3ywyde', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-26 16:04:23', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '新增', 'F110', '0', '@bpm/views/admin/org/FormV', '/api/v1/core/org/add', 'POST', '0', '部门管理 > 新增', 'D', '', '', '', '', '', 'info', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWL8AS3LRYGEK9CERV6UPQD', '03CWL8AS3LRYGEK9CET5ERTP4', '03cv1s494q7oqwk231b3ywyde', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-26 16:18:43', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '编辑', 'F120', '0', '@bpm/views/admin/org/FormV', '/api/v1/core/org/edit', 'PUT', '0', '部门管理  > 编辑', 'D', '', '', '', '', '', 'warning', '1', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWL9JLRCKY894HTIV85MY1X', '03CWL9JLRCKY894HTIXRP3GW2', '03cv1s494q7oqwk231b3ywyde', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-26 16:25:05', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '浏览', 'F130', '0', '@bpm/views/admin/org/FormV', '', 'GET', '0', '部门管理 > 浏览', 'D', '', '', '', '', '', 'primary', '1', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWMBRWIG2E1NHKEXZ2QZVTL', '03CWMBRWIG2E1NHKEXZN2RD4W', '03cv1s494q7oqwk231b3ywyde', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-26 19:40:47', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '删除', 'F140', '0', '', '/api/v1/core/org/remove', 'PUT', '0', '部门管理 > 删除', 'D', '', '', '', '', '', 'error', '2', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWNHLIBNAU7Y70VP9RJ9ZNZ', '03CWNHLIBNAU7Y70VPATU1U0X', '03cv1s4ummhh1caqebzlelian', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-26 23:14:53', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '新增', 'F110', '0', '@bpm/views/admin/role/FormV', '/api/v1/core/role/add', 'POST', '0', '角色管理 > 新增', '', null, null, null, null, '', 'info', '0', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWQOP0VFCMPGZTX2C3PDW2K', '03CWQOP0VFCMPGZTX2DV7IO2N', '03cv1s4ummhh1caqebzlelian', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 09:04:05', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '编辑', 'F120', '0', '@bpm/views/admin/role/FormV', '/api/v1/core/role/edit', 'PUT', '0', '角色管理 > 编辑', '', null, null, null, null, '', 'warning', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWQQ2D8WJXXKDDKUB0NVMII', '03CWQQ2D8WJXXKDDKUCBACNRY', '03cv1s4ummhh1caqebzlelian', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 09:11:06', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '浏览', 'F130', '0', '@bpm/views/admin/role/FormV', '', 'GET', '0', '角色管理 > 浏览', '', null, null, null, null, '', 'primary', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWQQAODNO10LCT0J7TL1VV4', '03CWQQAODNO10LCT0JAO5DQJ1', '03cv1s4ummhh1caqebzlelian', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 09:12:17', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '删除', 'F140', '0', '', '/api/v1/core/role/remove', 'PUT', '0', '角色管理 > 删除', '', null, null, null, null, '', 'error', '2', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWQROTGPSIUZJ67PP9BH0X6', '03CWQROTGPSIUZJ67PQ7A20VH', '03cv1s5idb4cpvqshtj7vibtt', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 09:19:25', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '新增', 'F110', '0', '@bpm/views/admin/post/FormV', '/api/v1/core/post/add', 'POST', '0', '岗位管理 > 新增', '', null, null, null, null, '', 'info', '0', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWQRS2GUES7MW3T6DP3KJWY', '03CWQRS2GUES7MW3T6FQO9EYT', '03cv1s5idb4cpvqshtj7vibtt', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 09:19:53', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '编辑', 'F120', '0', '@bpm/views/admin/post/FormV', '/api/v1/core/post/edit', 'PUT', '0', '岗位管理 > 编辑', '', null, null, null, null, '', 'warning', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWQRW5A5DKR09JXCEHIKLYM', '03CWQRW5A5DKR09JXCG7Z89UY', '03cv1s5idb4cpvqshtj7vibtt', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 09:20:27', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '浏览', 'F130', '0', '@bpm/views/admin/post/FormV', '', 'GET', '0', '岗位管理 > 浏览', '', null, null, null, null, '', 'primary', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWQRZZY5ASX2ACLCOU763CM', '03CWQRZZY5ASX2ACLCRH1YIAZ', '03cv1s5idb4cpvqshtj7vibtt', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 09:21:00', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '删除', 'F140', '0', '', '/api/v1/core/post/remove', 'PUT', '0', '岗位管理 > 删除', '', null, null, null, null, '', 'error', '2', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWQSJG8ZK5X9RGOS7SB8A6R', '03CWQSJG8ZK5X9RGOS9UX3UE9', '03cv1s67tys8b1rp4i64n4pzy', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 09:23:46', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '新增', 'F110', '0', '@bpm/views/admin/duty/FormV', '/api/v1/core/duty/add', 'POST', '0', '职务管理 > 新增', '', null, null, null, null, '', 'info', '0', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWQSLSXVE5VREADUT0PMAB3', '03CWQSLSXVE5VREADUU0POE2A', '03cv1s67tys8b1rp4i64n4pzy', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 09:24:06', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '编辑', 'F120', '0', '@bpm/views/admin/duty/FormV', '/api/v1/core/duty/edit', 'PUT', '0', '职务管理 > 编辑', '', null, null, null, null, '', 'warning', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWQSORD7TB9IK6R68E065RA', '03CWQSORD7TB9IK6R6AIO3RWU', '03cv1s67tys8b1rp4i64n4pzy', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 09:24:31', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '浏览', 'F130', '0', '@bpm/views/admin/duty/FormV', '', 'GET', '0', '职务管理 > 浏览', '', null, null, null, null, '', 'primary', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWQSS4Q1HI6D7AJU82KN33N', '03CWQSS4Q1HI6D7AJUAD75M17', '03cv1s67tys8b1rp4i64n4pzy', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 09:25:00', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '删除', 'F140', '0', '', '/api/v1/core/duty/remove', 'PUT', '0', '职务管理 > 删除', '', null, null, null, null, '', 'error', '2', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWR3ULJZX4GPZK8UVPWX3S3', '03CWR3ULJZX4GPZK8UXK1GAOO', '03cv1sd1fzc27jtsnpiu7xvy5', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 10:21:40', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '删除', 'F150', '0', '', '/api/v1/core/log/login/delete', 'DELETE', '0', '登录日志 > 删除', '', null, null, null, null, '', 'error', '2', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWR508FPOAY8GT97QCFQTWE', '03CWR508FPOAY8GT97SHKN2BL', '03cv1sd1fzc27jtsnpiu7xvy5', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 10:27:35', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '清空', 'F159', '0', '', '/api/v1/core/log/login/clean', 'DELETE', '0', '登录日志 > 清空', '', null, null, null, null, '', 'error', '0', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWS64CJ4650YA3L9OGNVOZS', '03CWS64CJ4650YA3L9PAEMHNF', '03cv1sfv869o7zrt8ls9xqxbm', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 13:37:35', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '强制下线', 'F301', '0', '', '/api/v1/core/online/delete', 'DELETE', '0', '在线用户 > 强制下线', '', null, null, null, null, '', 'error', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWSA5UDKVU09CM746SH3HVO', '03CWSA5UDKVU09CM74996UZ2C', '03cv1st6s0hh0xnjb56ga7qtb', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 13:58:16', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '新增', 'F110', '0', '@bpm/views/admin/tree/FormV', '/api/v1/core/tree/add', 'POST', '0', '树型关联 > 新增', '', null, null, null, null, '', 'info', '0', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWSA8G98S6JFCM525U5Y17X', '03CWSA8G98S6JFCM527SYW9OR', '03cv1st6s0hh0xnjb56ga7qtb', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 13:58:38', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '编辑', 'F120', '0', '@bpm/views/admin/tree/FormV', '/api/v1/core/tree/edit', 'PUT', '0', '树型关联 > 编辑', '', null, null, null, null, '', 'warning', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWSACFRE62R6HFE1EN58FT0', '03CWSACFRE62R6HFE1FI7KBMC', '03cv1st6s0hh0xnjb56ga7qtb', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 13:59:12', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '浏览', 'F130', '0', '@bpm/views/admin/tree/FormV', '', 'GET', '0', '树型关联 > 浏览', '', null, null, null, null, '', 'primary', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWSAGJQV3UWO1BSO5Z0Y0I5', '03CWSAGJQV3UWO1BSO9PJGAIQ', '03cv1st6s0hh0xnjb56ga7qtb', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 13:59:47', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '删除', 'F140', '0', '', '/api/v1/core/tree/remove', 'PUT', '0', '树型关联 > 删除', '', null, null, null, null, '', 'error', '2', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWSI2GUD3CZYUT8P1Z11SO8', '03CWSI2GUD3CZYUT8P42M1MNR', '03cv1sn88j69i3026fc36neap', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 14:38:44', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '新增', 'F110', '0', '@bpm/views/admin/dict/FormDataV', '/api/v1/core/dict/data/add', 'POST', '0', '字典数据 > 新增', '', null, null, null, null, '', 'info', '0', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWSI5A68YA1HGOVZUPZ20ZB', '03CWSI5A68YA1HGOVZXRPRD3N', '03cv1sn88j69i3026fc36neap', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 14:39:09', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '编辑', 'F120', '0', '@bpm/views/admin/dict/FormDataV', '/api/v1/core/dict/data/edit', 'PUT', '0', '字典数据 > 编辑', '', null, null, null, null, '', 'warning', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWSI8R7R3ZY6ORUZD71DP8J', '03CWSI8R7R3ZY6ORUZE93Z5YO', '03cv1sn88j69i3026fc36neap', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 14:39:38', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '浏览', 'F130', '0', '@bpm/views/admin/dict/FormDataV', '', 'GET', '0', '字典数据 > 浏览', '', null, null, null, null, '', 'primary', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWSL265K41YUVD625C63L0W', '03CWSL265K41YUVD62678BU2F', '03cv1sn88j69i3026fc36neap', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-27 14:54:03', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '删除', 'F140', '0', '', '/api/v1/core/dict/data/remove', 'PUT', '0', '字典数据 > 删除', '', null, null, null, null, '', 'error', '2', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWZ5SH1KRT4PE0550SV9GM0', '03CWZ5SH1KRT4PE0553ITJT8P', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-28 11:05:55', null, null, '1', '340', '1', '', '03ctywcl3sjqt4l75nzfh495n', '动态SQL', 'C', '0', '@bpm/views/TableView', '/api/v1/core/dql/find_all', 'GET', '0', '开发后台 -> 动态SQL', 'D', '@bpm/views/admin/dql/QueryV', '', '@bpm/views/admin/list/dql_list', '', '', '', '0', '', '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWZ7ZHFNE2WEJNWYT5B1V0T', '03CWZ7ZHFNE2WEJNWYUICBRPU', '03CWZ5SH1KRT4PE0553ITJT8P', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-28 11:17:09', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '新增', 'F110', '0', '@bpm/views/admin/dql/FormV', '/api/v1/core/dql/add', 'POST', '0', '动态SQL > 新增', '', null, null, null, null, '', 'info', '0', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWZ830GUE2A5BUBS861DS7N', '03CWZ830GUE2A5BUBS8OYI5SO', '03CWZ5SH1KRT4PE0553ITJT8P', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-28 11:17:39', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '编辑', 'F120', '0', '@bpm/views/admin/dql/FormV', '/api/v1/core/dql/edit', 'PUT', '0', '动态SQL >  编辑', '', null, null, null, null, '', 'warning', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWZ85KR1ZCSLUBMF6P327K6', '03CWZ85KR1ZCSLUBMF8LZBD0K', '03CWZ5SH1KRT4PE0553ITJT8P', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-28 11:18:01', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '浏览', 'F130', '0', '@bpm/views/admin/dql/FormV', '', 'PUT', '0', '动态SQL >  浏览', '', null, null, null, null, '', 'primary', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CWZ8CVVCU4EH5IDEHILP9FX', '03CWZ8CVVCU4EH5IDEJBH7AFC', '03CWZ5SH1KRT4PE0553ITJT8P', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-11-28 11:19:03', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '删除', 'F140', '0', '', '/api/v1/core/dql/remove', 'PUT', '0', '动态SQL > 删除', '', null, null, null, null, '', 'error', '2', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CY36N45WHECDPJJL2IU955E', '03CY36N45WHECDPJJL4YGAUQL', '03cv1oxlksezymqizbw2d0psz', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-03 14:01:50', '2024-12-05 12:50:20', null, '2', '104', '1', '', '03ctywcl3sjqt4l75nzfh495n', '功能管理', 'C', '0', '@bpm/views/TableView', '/api/v1/core/menu/find_fn_all', 'GET', '0', '开发后台 > 功能管理', '', '@bpm/views/admin/menu/QueryFnV', '', '@bpm/views/admin/list/menu_fn_list', '', '', '', '0', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CY37GU1HCYRIM7EFT9Z4H9J', '03CY37GU1HCYRIM7EFVOGNWY5', '03CY36N45WHECDPJJL4YGAUQL', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 14:06:03', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '新增', 'F110', '0', '@bpm/views/admin/menu/FnFormV', '/api/v1/core/menu/add', 'POST', '0', '功能管理 > 新增', '', null, null, null, null, '', 'info', '0', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CY37VBPU6FRVETGQ9BJHDVD', '03CY37VBPU6FRVETGQBZTGDPO', '03CY36N45WHECDPJJL4YGAUQL', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 14:08:07', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '编辑', 'F120', '0', '@bpm/views/admin/menu/FnFormV', '/api/v1/core/menu/edit', 'PUT', '0', '功能管理 > 编辑', '', null, null, null, null, '', 'warning', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CY38FDKUMDD8WCSG17QP00C', '03CY38FDKUMDD8WCSG3B7VYB4', '03CY36N45WHECDPJJL4YGAUQL', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 14:10:58', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '浏览', 'F130', '0', '@bpm/views/admin/menu/FnFormV', '', 'GET', '0', '功能管理 > 浏览', '', null, null, null, null, '', 'primary', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CY38JEHAFCMSTGKDRCDM6HC', '03CY38JEHAFCMSTGKDSRJ5U5G', '03CY36N45WHECDPJJL4YGAUQL', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-03 14:11:33', null, null, '1', '0', '1', '', '03ctywcl3sjqt4l75nzfh495n', '删除', 'F140', '0', '', '/api/v1/core/menu/delete', 'DELETE', '0', '功能管理 > 删除', '', null, null, null, null, '', 'error', '2', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYB9A3JAS5UU5PDXNKTWWX3', '03CYB9A3JAS5UU5PDXOY86TSY', '03cv1sb745kzhmvs78jl9xm6t', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-04 14:49:39', '2024-12-04 17:21:14', null, '3', '1', '1', '', '03ctywcl3sjqt4l75nzfh495n', '新增', 'F110', '0', '@bpm/views/admin/auth/FnForm', '/api/v1/core/auth/fn/add', 'POST', '0', '功能授权 > 新增', '', null, null, null, null, '', 'info', '0', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYB9FU7LAJK3B65ONBADCVK', '03CYB9FU7LAJK3B65OQJMDNRJ', '03cv1sb745kzhmvs78jl9xm6t', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-04 14:50:28', '2024-12-04 17:21:25', null, '2', '3', '1', '', '03ctywcl3sjqt4l75nzfh495n', '浏览', 'F130', '0', '@bpm/views/admin/auth/FnFormV', '', 'PUT', '0', '功能授权 > 浏览', '', null, null, null, null, '', 'primary', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYB9LB5TAUU52YXL0Y0VTMP', '03CYB9LB5TAUU52YXL2S6S9JM', '03cv1sb745kzhmvs78jl9xm6t', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-04 14:51:14', '2024-12-04 17:21:35', null, '4', '5', '1', '', '03ctywcl3sjqt4l75nzfh495n', '删除', 'F150', '0', '', '/api/v1/core/auth/fn/delete', 'DELETE', '0', '功能授权 > 删除', '', null, null, null, null, '', 'error', '2', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYC2RLX9LAFJOI5O3BPJIK1', '03CYC2RLX9LAFJOI5O52A0134', '03cv1sb745kzhmvs78jl9xm6t', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-04 17:20:35', '2024-12-04 17:21:20', null, '2', '2', '1', '', '03ctywcl3sjqt4l75nzfh495n', '编辑', 'F120', '0', '@bpm/views/admin/auth/FnFormV', '/api/v1/core/auth/fn/edit', 'PUT', '0', '功能授权 > 编辑', '', null, null, null, null, '', 'warning', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYH963IDWXUSVLBD4FA5T5U', '03CYH963IDWXUSVLBD7ENPDG3', '03cv1sahb19swyi9afdnujp36', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-05 09:14:49', null, null, '1', '1', '1', '', '03ctywcl3sjqt4l75nzfh495n', '新增', 'F110', '0', '@bpm/views/admin/auth/RowFormV', '/api/v1/core/auth/row/add', 'POST', '0', '行级授权 > 新增', '', null, null, null, null, '', 'info', '0', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYH998PMHC41BIGU93ZNPMJ', '03CYH998PMHC41BIGUB9T0XVP', '03cv1sahb19swyi9afdnujp36', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-05 09:15:15', '2024-12-05 09:18:24', null, '2', '2', '1', '', '03ctywcl3sjqt4l75nzfh495n', '编辑', 'F120', '0', '@bpm/views/admin/auth/RowFormV', '/api/v1/core/auth/row/edit', 'PUT', '0', '行级授权 > 编辑', '', null, null, null, null, '', 'warning', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYH9C4AYRCLHJ6XHAOFU79S', '03CYH9C4AYRCLHJ6XHBW0H89G', '03cv1sahb19swyi9afdnujp36', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-05 09:15:40', null, null, '1', '2', '1', '', '03ctywcl3sjqt4l75nzfh495n', '浏览', 'F130', '0', '@bpm/views/admin/auth/RowFormV', '', 'GET', '0', '行级授权 > 浏览', '', null, null, null, null, '', 'primary', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYH9I40NPYYZPYU1SV0H7P6', '03CYH9I40NPYYZPYU1W835DB0', '03cv1sahb19swyi9afdnujp36', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-05 09:16:31', null, null, '1', '2', '1', '', '03ctywcl3sjqt4l75nzfh495n', '删除', 'F140', '0', '', '/api/v1/core/auth/row/remove', 'PUT', '0', '行级授权 > 删除', '', null, null, null, null, '', 'error', '2', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYHD7Q8IQ4SKNU2PO5S99HZ', '03CYHD7Q8IQ4SKNU2PPYQQQHY', '03cv1s9wen3gcoh0qp8jmyuj4', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-05 09:35:31', '2024-12-05 09:42:36', null, '2', '1', '1', '', '03ctywcl3sjqt4l75nzfh495n', '新增', 'F110', '0', '@bpm/views/admin/auth/ColFormV', '/api/v1/core/auth/col/add', 'POST', '0', '列级授权 > 新增', '', null, null, null, null, '', 'info', '0', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYHDA7AOMZVE2D8LMOQL0N6', '03CYHDA7AU3UY5V66D5BKQH1S', '03cv1s9wen3gcoh0qp8jmyuj4', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-05 09:35:52', '2024-12-05 09:42:47', null, '2', '2', '1', '', '03ctywcl3sjqt4l75nzfh495n', '编辑', 'F120', '0', '@bpm/views/admin/auth/ColFormV', '/api/v1/core/auth/col/edit', 'PUT', '0', '列级授权 > 编辑', '', null, null, null, null, '', 'warning', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYHDD9LC2ZT55ZDZ6QA5L6Q', '03CYHDD9LC2ZT55ZDZ9LYZ18O', '03cv1s9wen3gcoh0qp8jmyuj4', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-05 09:36:18', '2024-12-05 09:42:55', null, '2', '3', '1', '', '03ctywcl3sjqt4l75nzfh495n', '浏览', 'F130', '0', '@bpm/views/admin/auth/ColFormV', '', 'GET', '0', '列级授权 > 浏览', '', null, null, null, null, '', 'primary', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYHDFYDFJLCJFVIUMIPWXD7', '03CYHDFYDFJLCJFVIUNLQ34YG', '03cv1s9wen3gcoh0qp8jmyuj4', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-05 09:36:41', '2024-12-05 09:43:02', null, '2', '5', '1', '', '03ctywcl3sjqt4l75nzfh495n', '删除', 'F150', '0', '', '/api/v1/core/auth/Col/delete', 'DELETE', '0', '列级授权 > 删除', '', null, null, null, null, '', 'error', '2', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYHHD2P5MZ03FFXH3ZMLF8C', '03CYHHD2P5MZ03FFXH4TEUTDL', '03cv1shm07ak44u24iz88yegx', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-05 09:56:45', null, null, '1', '1', '1', '', '03ctywcl3sjqt4l75nzfh495n', '新增', 'F110', '0', '@bpm/views/admin/adtion/FormExV', '/api/v1/core/adtion/type/add', 'POST', '0', '附件类别 > 新增', '', null, null, null, null, '', 'info', '0', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYHHFLR8TYBRNRGB0HH6T4M', '03CYHHFLR8TYBRNRGB3HLF1V8', '03cv1shm07ak44u24iz88yegx', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-05 09:57:07', null, null, '1', '2', '1', '', '03ctywcl3sjqt4l75nzfh495n', '编辑', 'F120', '0', '@bpm/views/admin/adtion/FormExV', '/api/v1/core/adtion/type/edit', 'PUT', '0', '附件类别 > 编辑', '', null, null, null, null, '', 'warning', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYHHHPDKK0CAM4VN8UHUKHY', '03CYHHHPDKK0CAM4VNAQKNF5F', '03cv1shm07ak44u24iz88yegx', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-05 09:57:25', null, null, '1', '3', '1', '', '03ctywcl3sjqt4l75nzfh495n', '浏览', 'F130', '0', '@bpm/views/admin/adtion/FormExV', '', 'PUT', '0', '附件类别 > 浏览', '', null, null, null, null, '', 'primary', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYHHKUF9HI83PWZN1ZAHY0X', '03CYHHKUF9HI83PWZN3G0XAFH', '03cv1shm07ak44u24iz88yegx', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-05 09:57:51', null, null, '1', '4', '1', '', '03ctywcl3sjqt4l75nzfh495n', '删除', 'F140', '0', '', '/api/v1/core/adtion/type/remove', 'PUT', '0', '附件类别 > 删除', '', null, null, null, null, '', 'error', '2', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYHO2JS9MWB6U8AM6OKORVA', '03CYHO2JS9MWB6U8AM82EU808', '03cv1sh19h7eguyipnnj1s9xc', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-05 10:31:05', '2024-12-05 10:31:52', null, '2', '3', '1', '', '03ctywcl3sjqt4l75nzfh495n', '浏览', 'F130', '0', '@bpm/views/admin/adtion/FormV', '', 'PUT', '0', '附件数据 > 浏览', '', null, null, null, null, '', 'primary', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYHO5QCM4EVI8ECWJNY63DU', '03CYHO5QCM4EVI8ECWLDPEXX2', '03cv1sh19h7eguyipnnj1s9xc', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-05 10:31:33', '2024-12-05 10:32:22', null, '3', '5', '1', '', '03ctywcl3sjqt4l75nzfh495n', '删除', 'F150', '0', '', '/api/v1/core/adtion/data/delete', 'DELETE', '0', '附件数据 > 删除', '', null, null, null, null, '', 'error', '2', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYHQ88B58TZH8YGVAXRL86G', '03CYHQ88B58TZH8YGVE2YOGAY', '03cv1su0vikj86xs1ywslvkge', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-05 10:42:08', null, null, '1', '1', '1', '', '03ctywcl3sjqt4l75nzfh495n', '新增', 'F110', '0', '@bpm/views/admin/md/FormV', '/api/v1/core/md/add', 'POST', '0', '主从关联 > 新增', '', null, null, null, null, '', 'info', '0', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYHQADS12Z3R5IAP0MV5Y2G', '03CYHQADS12Z3R5IAP1XF55WO', '03cv1su0vikj86xs1ywslvkge', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-05 10:42:27', null, null, '1', '2', '1', '', '03ctywcl3sjqt4l75nzfh495n', '编辑', 'F120', '0', '@bpm/views/admin/md/FormV', '/api/v1/core/md/edit', 'PUT', '0', '主从关联 > 编辑', '', null, null, null, null, '', 'warning', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYHQCBQRGSSRLO3HSGQ5UGB', '03CYHQCBQRGSSRLO3HTJUQLOG', '03cv1su0vikj86xs1ywslvkge', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-05 10:42:43', null, null, '1', '3', '1', '', '03ctywcl3sjqt4l75nzfh495n', '浏览', 'F130', '0', '@bpm/views/admin/md/FormV', '', 'GET', '0', '主从关联 > 浏览', '', null, null, null, null, '', 'primary', '1', null, '0', '', '', '', '', '');
INSERT INTO `bxy_menu` VALUES ('03CYHQG0L3KL0OHINZWTV7NGS', '03CYHQG0L3KL0OHINZYCYHZZJ', '03cv1su0vikj86xs1ywslvkge', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-05 10:43:15', null, null, '1', '5', '1', '', '03ctywcl3sjqt4l75nzfh495n', '删除', 'F150', '0', '', '/api/v1/core/md/delete', 'DELETE', '0', '主从关联 > 删除', '', null, null, null, null, '', 'error', '2', null, '0', '', '', '', '', '');

-- ----------------------------
-- Table structure for `bxy_online`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_online`;
CREATE TABLE `bxy_online` (
  `id` varchar(255) NOT NULL,
  `u_id` varchar(255) NOT NULL,
  `token_id` varchar(255) NOT NULL,
  `token_exp` bigint(20) NOT NULL,
  `uname` varchar(255) NOT NULL,
  `net` varchar(255) NOT NULL,
  `ip` varchar(255) NOT NULL,
  `login_location` varchar(255) NOT NULL,
  `device` varchar(255) NOT NULL,
  `browser` varchar(255) NOT NULL,
  `os` varchar(255) NOT NULL,
  `login_time` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_online
-- ----------------------------
INSERT INTO `bxy_online` VALUES ('03csomf3vebou7ta3dvp3fovt', '1', '03csomeytv8h8vixjt0nn3tk6', '1731918374', '系统管理员', '电信', '114.221.240.36', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-08 16:26:16');
INSERT INTO `bxy_online` VALUES ('03csomflwlblnnd0mswooqe1e', '1', '03csomfg6vpzh5sw4rt8y854t', '1731918378', '系统管理员', '电信', '114.221.240.36', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-08 16:26:20');
INSERT INTO `bxy_online` VALUES ('03csomk6wgnz1q76u0huxu6qh', '1', '03csomk4fr3u5oq59q1s0oy6j', '1731918418', '系统管理员', '电信', '114.221.240.36', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-08 16:26:59');
INSERT INTO `bxy_online` VALUES ('03csomy5e5doq8o52gdo8kp1t', '1', '03csomy2kzerjav3lf3wzte83', '1731918537', '系统管理员', '电信', '114.221.240.36', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-08 16:28:58');
INSERT INTO `bxy_online` VALUES ('03cson5jq88a3rvhvwvk0koel', '1', '03cson5fwwrwm8fyr5i1hxktx', '1731918600', '系统管理员', '电信', '114.221.240.36', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-08 16:30:01');
INSERT INTO `bxy_online` VALUES ('03csonc59bqqiudqng25gk216', '1', '03csonc0livfyo2xlkl7kvghk', '1731918656', '系统管理员', '电信', '114.221.240.36', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-08 16:30:58');
INSERT INTO `bxy_online` VALUES ('03csongwytq10mf0dzbjlv78c', '1', '03csongu0srhd4jo094nrplsc', '1731918697', '系统管理员', '电信', '114.221.240.36', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-08 16:31:38');
INSERT INTO `bxy_online` VALUES ('03csonlcz8hk671on7dphs0i1', '1', '03csonl9wi094x9ibn7ykejja', '1731918735', '系统管理员', '电信', '114.221.240.36', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-08 16:32:16');
INSERT INTO `bxy_online` VALUES ('03csons2iczu5ufpsptfttgh4', '1', '03csonryvkokzadadwywwor7r', '1731918792', '系统管理员', '电信', '114.221.240.36', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-08 16:33:13');
INSERT INTO `bxy_online` VALUES ('03csonvkc87irkneqnyg6ribs', '1', '03csonvfxn8ukptmd7o85d072', '1731918822', '系统管理员', '电信', '114.221.240.36', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-08 16:33:43');
INSERT INTO `bxy_online` VALUES ('03csoo9vp7641l74afwbhcv22', '1', '03csoo9qjpajof1j4zdro9a9e', '1731918944', '系统管理员', '电信', '114.221.240.36', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-08 16:35:45');
INSERT INTO `bxy_online` VALUES ('03csoop1qkpiaev6b93ohzaye', '1', '03csooowzex8878vhui597mm1', '1731919073', '系统管理员', '电信', '114.221.240.36', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-08 16:37:55');
INSERT INTO `bxy_online` VALUES ('03csosmkymwlig5nwn1uxbcrs', '1', '03csosmhjg0i68y3ardtm8hlz', '1731920281', '系统管理员', '电信', '114.221.240.36', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-08 16:58:02');
INSERT INTO `bxy_online` VALUES ('03csosyik2zmru4av1cw8v1tt', '1', '03csosyg18vet2017zvrpkelg', '1731920383', '系统管理员', '电信', '114.221.240.36', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-08 16:59:44');
INSERT INTO `bxy_online` VALUES ('03csottoo06afhc4by07dqw9i', '1', '03csottlkico0dfy4skowdnm6', '1731920649', '系统管理员', '电信', '114.221.240.36', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-08 17:04:10');
INSERT INTO `bxy_online` VALUES ('03cst65qx5g9krtfkvecuhrfm', '1', '03cst65n51qspd9mye741fthd', '1731968667', '系统管理员', '电信', '114.222.90.3', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-09 06:24:28');
INSERT INTO `bxy_online` VALUES ('03cta652yzs0vpntv55wofdhr', '1', '03cta64xynottl3obfutxptsv', '1732156635', '系统管理员', '电信', '114.221.240.113', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-11 10:37:17');
INSERT INTO `bxy_online` VALUES ('03ctih10sa4d9q788ewhjl2ym', '1', '03ctih0vj76oeesqc4alcfvrj', '1732248438', '系统管理员', '电信', '114.221.243.237', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-12 12:07:20');
INSERT INTO `bxy_online` VALUES ('03ctq0ra5omh8g15ihaqszgxn', '1', '03ctq0r5u1hjc759b98z0d24e', '1732331899', '系统管理员', '电信', '114.222.36.49', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-13 11:18:21');
INSERT INTO `bxy_online` VALUES ('03ctqhbexsc10os2q1739w0ci', '1', '03ctqhbcqhtew7qamwzpr8rak', '1732336986', '系统管理员', '电信', '114.222.36.49', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-13 12:43:07');
INSERT INTO `bxy_online` VALUES ('03ctys234n1tnwu6s4xeo0625', '1', '03ctys2002y5pfcl69j69lh5p', '1732428744', '系统管理员', '电信', '114.222.92.245', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-14 14:12:25');
INSERT INTO `bxy_online` VALUES ('03ctysi2mspapk9zb8xi1g7gw', '1', '03ctyshultxx24xr51dh4ctxw', '1732428879', '系统管理员', '电信', '114.222.92.245', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-14 14:14:41');
INSERT INTO `bxy_online` VALUES ('03ctysj32ychxxclz64iq070j', '1', '03ctysixr2dowi0gig3alamvg', '1732428888', '系统管理员', '电信', '114.222.92.245', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-14 14:14:50');
INSERT INTO `bxy_online` VALUES ('03ctyslczf0546w15tmw9rl56', '1', '03ctysl9lqsojo89gelfqu952', '1732428908', '系统管理员', '电信', '114.222.92.245', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-14 14:15:09');
INSERT INTO `bxy_online` VALUES ('03ctysy5dzqwuu1ow4ac2iq1g', '1', '03ctysxzya7qous7sui71r1kq', '1732429017', '系统管理员', '电信', '114.222.92.245', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-14 14:16:58');
INSERT INTO `bxy_online` VALUES ('03ctyt04t2pli4ng28v5ewoc8', '1', '03ctyt01me1ugwatkpvh5ctct', '1732429034', '系统管理员', '电信', '114.222.92.245', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-14 14:17:15');
INSERT INTO `bxy_online` VALUES ('03ctyt1kri6yitfi2pkwqsabj', '1', '03ctyt1heqsoewfhp16d6w9ha', '1732429046', '系统管理员', '电信', '114.222.92.245', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-14 14:17:28');
INSERT INTO `bxy_online` VALUES ('03ctyt5ne7kd6rerc3mxzfavs', '1', '03ctyt5jjiw80b0zpcpdfar83', '1732429081', '系统管理员', '电信', '114.222.92.245', '江苏省南京市', 'Other', 'Edge 130', 'Windows 10', '2024-11-14 14:18:02');
INSERT INTO `bxy_online` VALUES ('03cv0hqm8ufn8xtnoh1fmlf6y', '1', '03cv0hqg2qyokivoq94lyh0ia', '1732845752', '系统管理员', '电信', '114.222.36.78', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-19 10:02:34');
INSERT INTO `bxy_online` VALUES ('03cv1r0e1umjojv3c7ds9htz8', '1', '03cv1r07yawe9tqsglhjns0xo', '1732859657', '系统管理员', '电信', '114.222.36.78', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-19 13:54:19');
INSERT INTO `bxy_online` VALUES ('03cv2egtrddkftnyy8gj8948v', '1', '03cv2egq8znchj01k75da3s37', '1732866862', '系统管理员', '电信', '114.222.36.78', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-19 15:54:24');
INSERT INTO `bxy_online` VALUES ('03cv2en42r6vaohud92lv6oor', '1', '03cv2emzgoe93umfqwmoayr1p', '1732866916', '系统管理员', '电信', '114.222.36.78', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-19 15:55:17');
INSERT INTO `bxy_online` VALUES ('03cv2f5gjkaapmi3ojeyvfh02', '1', '03cv2f5e23dufr23fuzbdtob2', '1732867073', '系统管理员', '电信', '114.222.36.78', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-19 15:57:54');
INSERT INTO `bxy_online` VALUES ('03cv2ftesiymypgkkoox13o8t', '1', '03cv2ftbeur6e6m59ljm81f05', '1732867277', '系统管理员', '电信', '114.222.36.78', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-19 16:01:18');
INSERT INTO `bxy_online` VALUES ('03cv2g1kqzevwzebr4sl5bv02', '1', '03cv2g1hrfrpht5asyr3lybzn', '1732867347', '系统管理员', '电信', '114.222.36.78', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-19 16:02:28');
INSERT INTO `bxy_online` VALUES ('03cv2g9y9pudp0vui8oqivnvz', '1', '03cv2g9uguslfs7zzwoay4vec', '1732867418', '系统管理员', '电信', '114.222.36.78', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-19 16:03:39');
INSERT INTO `bxy_online` VALUES ('03cv2geud3whxg5o18ersjytp', '1', '03cv2gerruagqe8c6rhgl30sb', '1732867460', '系统管理员', '电信', '114.222.36.78', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-19 16:04:21');
INSERT INTO `bxy_online` VALUES ('03cv2ginjrx7v7jgeh1pmsoww', '1', '03cv2gij0xgu6m4juggwqb92g', '1732867492', '系统管理员', '电信', '114.222.36.78', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-19 16:04:54');
INSERT INTO `bxy_online` VALUES ('03cv2h562oxa8561tsz1qtesb', '1', '03cv2h52afqy9ytvglj6e70jp', '1732867684', '系统管理员', '电信', '114.222.36.78', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-19 16:08:06');
INSERT INTO `bxy_online` VALUES ('03cv86dr0nyc8x4d900te2r6w', '1', '03cv86djw70ajwnu24s4s91x7', '1732930722', '系统管理员', '电信', '114.222.131.131', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-20 09:38:44');
INSERT INTO `bxy_online` VALUES ('03cv86ma7n1hatvmgt21cbgoe', '1', '03cv86m77c1zht9k4xdfk5hb0', '1732930796', '系统管理员', '电信', '114.222.131.131', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-20 09:39:57');
INSERT INTO `bxy_online` VALUES ('03cv86rptp1eez62x50vly1h4', '1', '03cv86rk5lxerhm9vsjo98h55', '1732930842', '系统管理员', '电信', '114.222.131.131', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-20 09:40:43');
INSERT INTO `bxy_online` VALUES ('03cv8ljt18xpebn7l9ola936i', '1', '03cv8ljp06lazn7ehwzjwzrgn', '1732935382', '系统管理员', '电信', '114.222.131.131', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-20 10:56:23');
INSERT INTO `bxy_online` VALUES ('03cv8saztdaziu1bwmrbdo2js', '1', '03cv8sawxmgmtggud1wo8y4mq', '1732937457', '系统管理员', '电信', '114.222.131.131', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-20 11:30:58');
INSERT INTO `bxy_online` VALUES ('03cv8wmkp2t5kcds6nr6j7q08', '1', '03cv8wmgqz5r26kclogluctl6', '1732938784', '系统管理员', '电信', '114.222.131.131', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-20 11:53:05');
INSERT INTO `bxy_online` VALUES ('03cv8zht737icl4mvwdf35rku', '1', '03cv8zhndu1sz4tylybao32wa', '1732939664', '系统管理员', '电信', '114.222.131.131', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-20 12:07:46');
INSERT INTO `bxy_online` VALUES ('03cv8zmglfitsnhmegguzwuax', '1', '03cv8zmcgntvh3urv4shh6bzn', '1732939704', '系统管理员', '电信', '114.222.131.131', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-20 12:08:26');
INSERT INTO `bxy_online` VALUES ('03cv93esaimmbaeq66adyf8f8', '1', '03cv93emgaeoq5564hg4tpk7m', '1732940867', '系统管理员', '电信', '114.222.131.131', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-20 12:27:49');
INSERT INTO `bxy_online` VALUES ('03cv9i6uat5bcp4p4bxz1pc2l', '1', '03cv9i6s3283zxcq0fp4opz12', '1732945407', '系统管理员', '电信', '114.222.131.131', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-20 13:43:28');
INSERT INTO `bxy_online` VALUES ('03cvalbakcgi8g2yp0dg9eljq', '1', '03cvalb7gfawbl8rjcdv6nhq7', '1732957424', '系统管理员', '电信', '114.222.131.131', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-20 17:03:45');
INSERT INTO `bxy_online` VALUES ('03cvfqf163zlfb22hpoge255c', '1', '03cvfqeygl9e2rywzu448bboy', '1733014278', '系统管理员', '电信', '114.221.243.8', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-21 08:51:20');
INSERT INTO `bxy_online` VALUES ('03cvfqjvj7rntpnqa2lgg1gox', '1', '03cvfqjpwgrs3ka262jssovid', '1733014319', '系统管理员', '电信', '114.221.243.8', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-21 08:52:01');
INSERT INTO `bxy_online` VALUES ('03cvfr83lqdi6rl5je8cz2x4a', '1', '03cvfr7xxcmhlks2rjd66af0a', '1733014526', '系统管理员', '电信', '114.221.243.8', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-21 08:55:28');
INSERT INTO `bxy_online` VALUES ('03cvfuy43adzrykw2l0w0dwnc', '1', '03cvfuy1l258496clvdcj81pt', '1733015670', '系统管理员', '电信', '114.221.243.8', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-21 09:14:31');
INSERT INTO `bxy_online` VALUES ('03cvfy993p3ww0ftfpz1e2jfj', '1', '03cvfy96saxzotnlw8oc9763k', '1733016686', '系统管理员', '电信', '114.221.243.8', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-21 09:31:27');
INSERT INTO `bxy_online` VALUES ('03cvga8ps3ozmid4re472v15o', '1', '03cvga8mmew3hl628g0xt44xy', '1733020367', '系统管理员', '电信', '114.221.243.8', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-21 10:32:49');
INSERT INTO `bxy_online` VALUES ('03cvgud1jifp481le1mbkmb0v', '1', '03cvgucv9bviuxo98bsa4s8kb', '1733026546', '系统管理员', '电信', '114.221.243.8', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-21 12:15:48');
INSERT INTO `bxy_online` VALUES ('03cvhj1gjubyohh4bcq0eckrk', '1', '03cvhj1dt9bpskqe5g6f2mf58', '1733034127', '系统管理员', '电信', '114.221.243.8', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-21 14:22:08');
INSERT INTO `bxy_online` VALUES ('03cvhlw02u8r7co41aqd5dqxh', '1', '03cvhlvxu6iddz2aovb5m104e', '1733035002', '系统管理员', '电信', '114.221.243.8', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-21 14:36:43');
INSERT INTO `bxy_online` VALUES ('03cvhmc8bcatzgji7wl5uie1z', '1', '03cvhmc5x7pdegibu8pnjv03z', '1733035141', '系统管理员', '电信', '114.221.243.8', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-21 14:39:02');
INSERT INTO `bxy_online` VALUES ('03cvhnpm99xzixp7cw26d9274', '1', '03cvhnpjulsrtulnujs8l6a6t', '1733035562', '系统管理员', '电信', '114.221.243.8', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-21 14:46:03');
INSERT INTO `bxy_online` VALUES ('03cvhx137zrffzftx6mnxitqz', '1', '03cvhx0zikjc8fvgunkssd9yh', '1733038424', '系统管理员', '电信', '114.221.243.8', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-21 15:33:45');
INSERT INTO `bxy_online` VALUES ('03cvhx5ksteodx1z0qx2mwxle', '1', '03cvhx5i3aoh1e10fcoeqn1ad', '1733038462', '系统管理员', '电信', '114.221.243.8', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-21 15:34:24');
INSERT INTO `bxy_online` VALUES ('03cvhxftenl5izfno27g983bz', '1', '03cvhxfp022z5dzkx4z4o78qd', '1733038549', '系统管理员', '电信', '114.221.243.8', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-21 15:35:51');
INSERT INTO `bxy_online` VALUES ('03cvhzvmwc6nxk2nc5v5kd72x', '1', '03cvhzvk1i28w6dnwxhaznr5n', '1733039299', '系统管理员', '电信', '114.221.243.8', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-21 15:48:20');
INSERT INTO `bxy_online` VALUES ('03cvi3dgti2vl8ga6dp78ly6i', '1', '03cvi3deh73rxg0bi1x3pqmnt', '1733040373', '系统管理员', '电信', '114.221.243.8', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-21 16:06:14');
INSERT INTO `bxy_online` VALUES ('03cvi50qqs9ojddz2ofijy349', '1', '03cvi50n4jbrnfad1ltbzssgp', '1733040878', '系统管理员', '电信', '114.221.243.8', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-21 16:14:39');
INSERT INTO `bxy_online` VALUES ('03cvi5un60hfgo8vgljqrg135', '1', '03cvi5uhjmoqq67auio04fsnq', '1733041133', '系统管理员', '电信', '114.221.243.8', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-21 16:18:55');
INSERT INTO `bxy_online` VALUES ('03cvi656o7dbh9kh5dzi57ikz', '1', '03cvi650zwn9hyrp89kk1k8ys', '1733041223', '系统管理员', '电信', '114.221.243.8', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-21 16:20:25');
INSERT INTO `bxy_online` VALUES ('03cvi9hlxj6sr0zo048mx21nl', '1', '03cvi9hjiwzms3a2fgpeqgaj4', '1733042251', '系统管理员', '电信', '114.221.243.8', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-21 16:37:32');
INSERT INTO `bxy_online` VALUES ('03cvib1v68zblri6g1d0ykv95', '1', '03cvib1orhagqxxoxhuc5afc5', '1733042730', '系统管理员', '电信', '114.221.243.8', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-21 16:45:32');
INSERT INTO `bxy_online` VALUES ('03cvp4j4lcezu1wclwfnpriyc', '1', '03cvp4iy95uoitalgqwua03li', '1733118129', '系统管理员', '电信', '114.222.39.126', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-22 13:42:11');
INSERT INTO `bxy_online` VALUES ('03cvp4nqu6bs6zwuhsf2w15lj', '1', '03cvp4nnpypdbuvrojw6ymkss', '1733118169', '系统管理员', '电信', '114.222.39.126', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-22 13:42:50');
INSERT INTO `bxy_online` VALUES ('03cvpbkh3ndnsor5fcarz7fkd', '1', '03cvpbkcn4ssh75jzub4gqewh', '1733120290', '系统管理员', '电信', '114.222.39.126', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-22 14:18:12');
INSERT INTO `bxy_online` VALUES ('03cvpboe2zv32vcnkqgx7p3tm', '1', '03cvpbo8q4qfrqvk2su9xx9an', '1733120324', '系统管理员', '电信', '114.222.39.126', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-22 14:18:45');
INSERT INTO `bxy_online` VALUES ('03cvpbpg2vcpxkh385h5usdcb', '1', '03cvpbpcd21v173hmhjb391qi', '1733120333', '系统管理员', '电信', '114.222.39.126', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-22 14:18:54');
INSERT INTO `bxy_online` VALUES ('03CWB392QIGI7ME2E7A09IVJH', '1', '03cwb38x4h4rq0gmbotvbz92r', '1733360997', '系统管理员', '电信', '117.89.216.20', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-25 09:09:59');
INSERT INTO `bxy_online` VALUES ('03CWB3Y8B4UB7DJBTN5TDG7PM', '1', '03cwb3y4txgp8gi1hic8amo9y', '1733361212', '系统管理员', '电信', '117.89.216.20', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-25 09:13:34');
INSERT INTO `bxy_online` VALUES ('03CWDEC5T195FEN9953O2F2V6', '1', '03cwdec3lw7edpek723r5oncx', '1733386518', '系统管理员', '电信', '117.89.216.20', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-25 16:15:19');
INSERT INTO `bxy_online` VALUES ('03CWS5TIZ5ZDM29AJL5ZUF9BY', '1', '03cws5tdx9u623p8hgv98d68t', '1733549761', '系统管理员', '电信', '114.222.172.113', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-11-27 13:36:02');
INSERT INTO `bxy_online` VALUES ('03CY9W4F2SC8UENL09F7UJFVU', '1', '03cy9w4c8k3a4309dsgz5a0eu', '1734143879', '系统管理员', '电信', '117.89.216.20', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-12-04 10:38:00');
INSERT INTO `bxy_online` VALUES ('03CYA02QKZWLITT5R5FW1FB9R', '1', '03cya02l3es4n1ghk8nulzx8q', '1734145092', '系统管理员', '电信', '117.89.216.20', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-12-04 10:58:14');
INSERT INTO `bxy_online` VALUES ('03CYQ1J5E44MAVA822AT5IRV4', '1', '03cyq1j3ax9b944iqpbmsh30m', '1734322457', '系统管理员', '电信', '114.222.172.54', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-12-06 12:14:18');
INSERT INTO `bxy_online` VALUES ('03CYQ9LTJEK51L1VBOFPYQZF1', '1', '03cyq9loohc5q1g490ej7uzzp', '1734324937', '系统管理员', '电信', '114.222.172.54', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-12-06 12:55:38');
INSERT INTO `bxy_online` VALUES ('03CYQB7AD95G8A051WD4BIZ1G', '1', '03cyqb77s50a3zjpqtvgz2htw', '1734325428', '系统管理员', '电信', '114.222.172.54', '江苏省南京市', 'Other', 'Edge 131', 'Windows 10', '2024-12-06 13:03:49');

-- ----------------------------
-- Table structure for `bxy_oper_log`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_oper_log`;
CREATE TABLE `bxy_oper_log` (
  `id` varchar(255) NOT NULL,
  `time_id` bigint(20) NOT NULL,
  `title` varchar(255) NOT NULL,
  `business_type` varchar(255) NOT NULL,
  `method` varchar(255) NOT NULL,
  `request_method` varchar(255) NOT NULL,
  `operator_type` varchar(255) NOT NULL,
  `oper_name` varchar(255) NOT NULL,
  `oper_url` varchar(255) NOT NULL,
  `oper_ip` varchar(255) NOT NULL,
  `oper_location` varchar(255) NOT NULL,
  `oper_param` varchar(255) NOT NULL,
  `path_param` varchar(255) NOT NULL,
  `json_result` varchar(255) NOT NULL,
  `status` varchar(255) NOT NULL,
  `error_msg` varchar(255) NOT NULL,
  `duration` bigint(20) NOT NULL,
  `oper_time` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_oper_log
-- ----------------------------

-- ----------------------------
-- Table structure for `bxy_org`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_org`;
CREATE TABLE `bxy_org` (
  `id` varchar(255) NOT NULL,
  `guid` varchar(255) NOT NULL,
  `pguid` varchar(255) NOT NULL,
  `create_by` varchar(255) NOT NULL,
  `update_by` varchar(255) DEFAULT NULL,
  `delete_by` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime DEFAULT NULL,
  `deleted_at` datetime DEFAULT NULL,
  `version` int(10) unsigned NOT NULL,
  `ord` int(10) unsigned NOT NULL,
  `status` varchar(255) NOT NULL,
  `remark` varchar(255) DEFAULT NULL,
  `oname` varchar(255) NOT NULL,
  `leader` varchar(255) DEFAULT NULL,
  `email` varchar(255) DEFAULT NULL,
  `phone` varchar(255) DEFAULT NULL,
  `att` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_org
-- ----------------------------
INSERT INTO `bxy_org` VALUES ('03CWL7W06VMUWF233JSNY0HPE', '03CWL7W06VMUWF233JUL697ZH', '03CWL7W06VMUWF233JUL697ZH', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-26 16:16:36', '2024-12-06 13:12:08', null, '6', '0', '1', '', '豁豁哈', '豁豁哈', '56889758@qq.com', '13999999999', '总公司');
INSERT INTO `bxy_org` VALUES ('03CYB3E2S6FNWEX6BFRNHGH75', '03CYB3E2S6FNWEX6BFTY3P8IP', '03CWL7W06VMUWF233JUL697ZH', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-04 14:19:30', null, null, '1', '1', '1', '', '管理部', '', '', '', '');
INSERT INTO `bxy_org` VALUES ('03CYB3F6EYU691JV9JPLPUGGQ', '03CYB3F6EYU691JV9JSFXQ3RI', '03CWL7W06VMUWF233JUL697ZH', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-04 14:19:39', null, null, '1', '2', '1', '', '研发部', '', '', '', '');
INSERT INTO `bxy_org` VALUES ('03CYB3FRB1N3I3CWP5VTBIA15', '03CYB3FRB1N3I3CWP5XZGP8NS', '03CWL7W06VMUWF233JUL697ZH', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-04 14:19:44', '2024-12-04 14:20:07', null, '2', '3', '1', '', '产品部', '', '', '', '');
INSERT INTO `bxy_org` VALUES ('03CYB3GBDO3GQIB8D20PI987W', '03CYB3GBDO3GQIB8D23EMYHAI', '03CWL7W06VMUWF233JUL697ZH', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-04 14:19:49', '2024-12-04 14:20:12', null, '2', '4', '1', '', '项目部', '', '', '', '');

-- ----------------------------
-- Table structure for `bxy_post`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_post`;
CREATE TABLE `bxy_post` (
  `id` varchar(255) NOT NULL,
  `guid` varchar(255) NOT NULL,
  `pguid` varchar(255) NOT NULL,
  `create_by` varchar(255) NOT NULL,
  `update_by` varchar(255) DEFAULT NULL,
  `delete_by` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime DEFAULT NULL,
  `deleted_at` datetime DEFAULT NULL,
  `version` int(10) unsigned NOT NULL,
  `ord` int(10) unsigned NOT NULL,
  `status` varchar(255) NOT NULL,
  `remark` varchar(255) DEFAULT NULL,
  `pname` varchar(255) NOT NULL,
  `att` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_post
-- ----------------------------
INSERT INTO `bxy_post` VALUES ('03CWQS34X4FU3VS56TSJ9XRZJ', '03CWQS34X4FU3VS56TUE4NBMQ', '03CWQS34X4FU3VS56TUE4NBMQ', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-27 09:21:27', '2024-11-27 09:29:34', null, '2', '0', '1', '', '总经理', '管理岗');
INSERT INTO `bxy_post` VALUES ('03CYB48JGDWCGH7LFN5UWGTNX', '03CYB48JGDWCGH7LFN7RPG0CZ', '03CYB48JGDWCGH7LFN7RPG0CZ', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-04 14:23:49', '2024-12-04 14:25:53', null, '2', '0', '1', '', '经理', '');

-- ----------------------------
-- Table structure for `bxy_role`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_role`;
CREATE TABLE `bxy_role` (
  `id` varchar(255) NOT NULL,
  `guid` varchar(255) NOT NULL,
  `pguid` varchar(255) NOT NULL,
  `create_by` varchar(255) NOT NULL,
  `update_by` varchar(255) DEFAULT NULL,
  `delete_by` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime DEFAULT NULL,
  `deleted_at` datetime DEFAULT NULL,
  `version` int(10) unsigned NOT NULL,
  `ord` int(10) unsigned NOT NULL,
  `status` varchar(255) NOT NULL,
  `remark` varchar(255) DEFAULT NULL,
  `rname` varchar(255) NOT NULL,
  `att` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_role
-- ----------------------------
INSERT INTO `bxy_role` VALUES ('03CWQNM40D64O4HZDDTEUS46J', '0000000000', '0000000000', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-11-27 08:58:33', '2024-11-27 09:07:02', null, '2', '0', '1', '系统最高权限角色，拥有所有权限', '系统管理员', '');
INSERT INTO `bxy_role` VALUES ('03CYB3WNYOLZBIGCK14VC0GGG', '9900000000', '9900000000', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-04 14:22:08', null, null, '1', '1', '1', '', '管理人员', '');
INSERT INTO `bxy_role` VALUES ('03CYB3ZWEGGSURAXVQAU79QDB', '9901000000', '9901000000', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-04 14:22:36', null, null, '1', '2', '1', '', '技术人员', '');

-- ----------------------------
-- Table structure for `bxy_row_auth`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_row_auth`;
CREATE TABLE `bxy_row_auth` (
  `id` varchar(255) NOT NULL,
  `create_by` varchar(255) NOT NULL,
  `update_by` varchar(255) DEFAULT NULL,
  `delete_by` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime DEFAULT NULL,
  `deleted_at` datetime DEFAULT NULL,
  `version` int(10) unsigned NOT NULL,
  `ord` int(10) unsigned NOT NULL,
  `status` varchar(255) NOT NULL,
  `remark` varchar(255) DEFAULT NULL,
  `title` varchar(255) NOT NULL,
  `content` varchar(255) NOT NULL,
  `mcode` varchar(255) NOT NULL,
  `atype` tinyint(3) unsigned NOT NULL,
  `amethod` smallint(5) unsigned NOT NULL,
  `u_id` varchar(255) NOT NULL,
  `r_id` varchar(255) NOT NULL,
  `o_id` varchar(255) NOT NULL,
  `p_id` varchar(255) NOT NULL,
  `d_id` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_row_auth
-- ----------------------------
INSERT INTO `bxy_row_auth` VALUES ('03CYHC1GEI6EFL4F8911X2Z3G', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-05 09:29:30', '2024-12-05 14:20:08', null, '3', '0', '1', '', '限制动态脚本BXY部分仅对管理员可见', 'sign like \'BXY%\'', '03CWZ5SH1KRT4PE0553ITJT8P', '0', '2', '', '0000000000', '', '', '');

-- ----------------------------
-- Table structure for `bxy_tree`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_tree`;
CREATE TABLE `bxy_tree` (
  `id` varchar(255) NOT NULL,
  `guid` varchar(255) NOT NULL,
  `pguid` varchar(255) NOT NULL,
  `create_by` varchar(255) NOT NULL,
  `update_by` varchar(255) DEFAULT NULL,
  `delete_by` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime DEFAULT NULL,
  `deleted_at` datetime DEFAULT NULL,
  `version` int(10) unsigned NOT NULL,
  `ord` int(10) unsigned NOT NULL,
  `status` varchar(255) NOT NULL,
  `remark` varchar(255) NOT NULL,
  `tv` varchar(255) NOT NULL,
  `sfield` varchar(255) NOT NULL,
  `twhere` varchar(255) NOT NULL,
  `tord` varchar(255) NOT NULL,
  `tname` varchar(255) NOT NULL,
  `mcode` varchar(255) NOT NULL,
  `tfields` varchar(255) NOT NULL,
  `mfields` varchar(255) NOT NULL,
  `atype` tinyint(3) unsigned NOT NULL,
  `amethod` tinyint(3) unsigned NOT NULL,
  `u_id` varchar(255) NOT NULL,
  `r_id` varchar(255) NOT NULL,
  `o_id` varchar(255) NOT NULL,
  `p_id` varchar(255) NOT NULL,
  `d_id` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_tree
-- ----------------------------
INSERT INTO `bxy_tree` VALUES ('03CXWMYPNILYY6TBQNKM24LJM', 'guid', 'pguid', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-02 17:55:21', '2024-12-03 19:02:30', null, '12', '0', '1', '', 'bxy_menu', 'mname', 'where mtype=\'C\' and (status=\'0\' or status=\'1\')', 'order by ord', 'BXY_MENU_TREE', '03cv1rwvjw9hqcmsxy5axxzyn', 'guid', 'guid', '0', '0', '', '', '', '', '');
INSERT INTO `bxy_tree` VALUES ('03CY9NWPTH92E3XSBSJ5575MR', 'guid', 'pguid', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-04 09:55:57', null, null, '1', '0', '1', '', 'bxy_menu', 'mname', 'where mtype=\'C\' and (status=\'0\' or status=\'1\')', 'order by ord', 'BXY_FN_TREE', '03CY36N45WHECDPJJL4YGAUQL', 'guid', 'pguid', '0', '0', '', '', '', '', '');
INSERT INTO `bxy_tree` VALUES ('03CY9U07S7P76WB3VTAOITSC0', 'guid', 'pguid', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-04 10:27:10', '2024-12-04 10:36:43', null, '2', '0', '1', '', 'bxy_dict', 'dname', 'where (status=\'0\' or status=\'1\')', 'order by ord', 'BXY_DICT_TREE', '03cv1sn88j69i3026fc36neap', 'guid', 'dname', '0', '0', '', '', '', '', '');

-- ----------------------------
-- Table structure for `bxy_update_log`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_update_log`;
CREATE TABLE `bxy_update_log` (
  `id` varchar(255) NOT NULL,
  `app_version` varchar(255) NOT NULL,
  `backend_version` varchar(255) NOT NULL,
  `title` varchar(255) NOT NULL,
  `content` varchar(255) NOT NULL,
  `create_by` varchar(255) NOT NULL,
  `update_by` varchar(255) DEFAULT NULL,
  `delete_by` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime DEFAULT NULL,
  `deleted_at` datetime DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_update_log
-- ----------------------------

-- ----------------------------
-- Table structure for `bxy_user`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_user`;
CREATE TABLE `bxy_user` (
  `id` varchar(255) NOT NULL,
  `u_id` varchar(255) NOT NULL,
  `create_by` varchar(255) NOT NULL,
  `update_by` varchar(255) DEFAULT NULL,
  `delete_by` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime DEFAULT NULL,
  `deleted_at` datetime DEFAULT NULL,
  `version` int(10) unsigned NOT NULL,
  `ord` int(10) unsigned NOT NULL,
  `status` varchar(255) NOT NULL,
  `remark` varchar(255) DEFAULT NULL,
  `ucode` varchar(255) NOT NULL,
  `uname` varchar(255) NOT NULL,
  `upwd` varchar(255) NOT NULL,
  `sex` varchar(255) NOT NULL,
  `salt` varchar(255) NOT NULL,
  `email` varchar(255) DEFAULT NULL,
  `qq` varchar(255) DEFAULT NULL,
  `webchat` varchar(255) DEFAULT NULL,
  `phone` varchar(255) DEFAULT NULL,
  `pin` varchar(255) DEFAULT NULL,
  `pass` varchar(255) DEFAULT NULL,
  `avatar` varchar(255) NOT NULL,
  `last_login_ip` varchar(255) DEFAULT NULL,
  `last_login_time` datetime DEFAULT NULL,
  `ext1` varchar(255) DEFAULT NULL,
  `ext2` varchar(255) DEFAULT NULL,
  `ext3` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_user
-- ----------------------------
INSERT INTO `bxy_user` VALUES ('03CYB61OUXNT6QNRC0BXUSU44', '03CYB61OUXNT6QNRC0EBTYZZ6', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', '2024-12-04 14:33:05', '2024-12-05 13:42:06', '2024-12-06 13:11:17', '1', '0', '2', '', 'yutf', '太飞', '6e12217118ed98f31d7b168d0ca69b1f', '男', 'G25G1$B#Is', '', '', '', '', '1234', '1234', '', null, null, '', '', '');
INSERT INTO `bxy_user` VALUES ('03CYB651PGOIJCRSOAIG9WBSW', '03CYB651PGOIJCRSOALI7BQV8', '03cv1qojjnl1tjl2xyakbwpj3', '03cv1qojjnl1tjl2xyakbwpj3', null, '2024-12-04 14:33:34', '2024-12-04 14:33:51', null, '1', '0', '1', '', 'hhh', '豁豁哈', 'd3b5dd80f9c14b0cc807c224872e92a6', '男', 'GFdixpo3XS', '', '', '', '', '1234', '1234', '', null, null, '', '', '');
INSERT INTO `bxy_user` VALUES ('1', '03cv1qojjnl1tjl2xyakbwpj3', '', '03cv1qojjnl1tjl2xyakbwpj3', '1', '2024-11-08 15:59:24', '2024-11-27 10:12:03', null, '1', '1', '1', '无', 'admin', '系统管理员', '65dec30b535dec007ed7a67f9121d140', '男', 'e7zOJWEU!)', '12345678@qq.com', '12345678', '12345678', '13999999999', '1234', '1234', '1', '1', '2024-11-08 16:01:33', '1', '1', '1');

-- ----------------------------
-- Table structure for `bxy_u_api`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_u_api`;
CREATE TABLE `bxy_u_api` (
  `id` varchar(255) NOT NULL,
  `u_id` varchar(255) NOT NULL,
  `mcode` varchar(255) DEFAULT NULL,
  `api` varchar(255) NOT NULL,
  `method` varchar(255) NOT NULL,
  `created_at` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_u_api
-- ----------------------------

-- ----------------------------
-- Table structure for `bxy_u_auth`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_u_auth`;
CREATE TABLE `bxy_u_auth` (
  `id` varchar(255) NOT NULL,
  `guid` varchar(255) NOT NULL,
  `create_by` varchar(255) NOT NULL,
  `update_by` varchar(255) DEFAULT NULL,
  `delete_by` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime DEFAULT NULL,
  `deleted_at` datetime DEFAULT NULL,
  `version` int(10) unsigned NOT NULL,
  `ord` int(10) unsigned NOT NULL,
  `remark` varchar(255) DEFAULT NULL,
  `mcode` varchar(255) NOT NULL,
  `atype` tinyint(3) unsigned NOT NULL,
  `amethod` smallint(5) unsigned NOT NULL,
  `u_id` varchar(255) NOT NULL,
  `r_id` varchar(255) NOT NULL,
  `o_id` varchar(255) NOT NULL,
  `p_id` varchar(255) NOT NULL,
  `d_id` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_u_auth
-- ----------------------------
INSERT INTO `bxy_u_auth` VALUES ('03CYBUAJBD2MDRM1BMYAKWVYO', '03CYBUAJBD2MDRM1BMZYJOYEX', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-04 16:37:12', null, null, '1', '0', '', '03CWKT9HR52RTASIY8C3TI560', '0', '2', '', '0000000000', '', '', '');
INSERT INTO `bxy_u_auth` VALUES ('03CYBUAJSZERAR0OCKFFPDVYT', '03CYBUAJSZERAR0OCKH8XCN63', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-04 16:37:12', null, null, '1', '0', '', '03CWK4GOS061Y8AF149QKQPHQ', '0', '2', '', '0000000000', '', '', '');
INSERT INTO `bxy_u_auth` VALUES ('03CYBUAKGOHFAFP7ZL72I7IS1', '03CYBUAKGOHFAFP7ZL8ONIJKC', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-04 16:37:13', null, null, '1', '0', '', '03CWK4ILLVEQXXZHI0TPMFNL8', '0', '2', '', '0000000000', '', '', '');
INSERT INTO `bxy_u_auth` VALUES ('03CYBUAKY5CP4O598MPP0MUWM', '03CYBUAKY5CP4O598MRMH8EVU', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-04 16:37:13', null, null, '1', '0', '', '03CWK4M7CKOFEO25VXU36HOHW', '0', '2', '', '0000000000', '', '', '');
INSERT INTO `bxy_u_auth` VALUES ('03CYBUALLOYI1KSKUU34U39QG', '03CYBUALLOYI1KSKUU483S0CT', '03cv1qojjnl1tjl2xyakbwpj3', null, null, '2024-12-04 16:37:13', null, null, '1', '0', '', '03CWK4OJLTI24K0CPJCK4E48U', '0', '2', '', '0000000000', '', '', '');

-- ----------------------------
-- Table structure for `bxy_u_duty`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_u_duty`;
CREATE TABLE `bxy_u_duty` (
  `id` varchar(255) NOT NULL,
  `u_id` varchar(255) NOT NULL,
  `d_id` varchar(255) NOT NULL,
  `created_at` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_u_duty
-- ----------------------------

-- ----------------------------
-- Table structure for `bxy_u_org`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_u_org`;
CREATE TABLE `bxy_u_org` (
  `id` varchar(255) NOT NULL,
  `u_id` varchar(255) NOT NULL,
  `o_id` varchar(255) NOT NULL,
  `created_at` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_u_org
-- ----------------------------

-- ----------------------------
-- Table structure for `bxy_u_post`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_u_post`;
CREATE TABLE `bxy_u_post` (
  `id` varchar(255) NOT NULL,
  `u_id` varchar(255) NOT NULL,
  `p_id` varchar(255) NOT NULL,
  `created_at` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_u_post
-- ----------------------------

-- ----------------------------
-- Table structure for `bxy_u_role`
-- ----------------------------
DROP TABLE IF EXISTS `bxy_u_role`;
CREATE TABLE `bxy_u_role` (
  `id` varchar(255) NOT NULL,
  `u_id` varchar(255) NOT NULL,
  `r_id` varchar(255) NOT NULL,
  `created_at` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of bxy_u_role
-- ----------------------------

-- ----------------------------
-- Table structure for `seaql_migrations`
-- ----------------------------
DROP TABLE IF EXISTS `seaql_migrations`;
CREATE TABLE `seaql_migrations` (
  `version` varchar(255) NOT NULL,
  `applied_at` bigint(20) NOT NULL,
  PRIMARY KEY (`version`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of seaql_migrations
-- ----------------------------
INSERT INTO `seaql_migrations` VALUES ('m20220101_000001_create_table', '1731031269');
