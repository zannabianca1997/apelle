package io.github.zannabianca1997.apelle.search.services;

import org.jboss.logging.Logger;

import io.github.zannabianca1997.apelle.common.dtos.Page;
import io.github.zannabianca1997.apelle.common.dtos.PageRequest;
import io.github.zannabianca1997.apelle.search.dtos.SearchedSongQueryDto;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.github.zannabianca1997.apelle.users.services.UsersService;
import io.github.zannabianca1997.apelle.youtube.services.YoutubeService;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;

@ApplicationScoped
public class SearchService {

    @Inject
    Logger log;

    @Inject
    YoutubeService youtubeService;

    @Inject
    UsersService usersService;

    public Page<SearchedSongQueryDto> search(String query, PageRequest pageRequest) {
        ApelleUser current = usersService.getMe();

        log.infof("[user=%s] Searched `%s`", current.getId(), query);

        // Only source is youtube, delegating to it
        return youtubeService.search(query, pageRequest);
    }

}
