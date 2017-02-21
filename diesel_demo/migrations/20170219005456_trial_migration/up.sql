-- CREATE TABLE IF NOT EXISTS `network_super_account` (
--   `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
--   `network` varchar(32) NOT NULL DEFAULT '',
--   `user_id` varchar(128) NOT NULL DEFAULT '',
--   `username` varchar(128) NOT NULL DEFAULT '',
--   `created` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
--   `updated` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
--   `deleted` tinyint(1) DEFAULT '0',
--   `deleted_timestamp` int(10) unsigned NOT NULL DEFAULT '0',
--   PRIMARY KEY (`id`),
--   UNIQUE KEY `idx_pa_n_ui` (`network`,`user_id`)
-- ) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE IF NOT EXISTS `posts` (
  `id` int(20) unsigned NOT NULL AUTO_INCREMENT,
  `title` varchar(32) NOT NULL DEFAULT '',
  `body` varchar(255) NOT NULL DEFAULT '',
  `published` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
