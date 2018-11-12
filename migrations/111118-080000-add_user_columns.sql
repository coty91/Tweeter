/**
 * Author: Coty A. Rothery
 * Date: 11/11/2018
 */
 
ALTER TABLE public.users ADD password char(64) NOT NULL;
ALTER TABLE public.users ADD salt char(20) NOT NULL;